use std::collections::HashMap;
use std::ffi::CString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use log::{error, info};

use fork::{Fork, fork};
use nix::mount::{MntFlags, MsFlags, mount, umount2};
use nix::sched::{self, CloneFlags};
use nix::sys::wait::{WaitStatus, waitpid};
use nix::unistd::{self, Pid};

use interprocess::os::unix as ipc_unix;
use interprocess::unnamed_pipe::{Recver, Sender};

use anyhow::{Context, Result, bail};

use crate::common::models::ProcessConfig;

fn hashmap_to_cstring_env(env_map: &HashMap<String, String>) -> Vec<CString> {
    env_map
        .iter()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect()
}

/*
 * This function is executed by the child process to setup its own isolations.
 *
 * We return immediately with error in case something goes wrong.
 * This is because something failed while talking to the kernel
 * and the error is not recoverable. This must be caught to do a proper
 * cleanup.
 */
pub fn setup_isoproc(pcfg: &ProcessConfig, recv: &mut Recver, sndr: &mut Sender) -> Result<()> {
    info!("setting up the isolated process");

    let new_root = format!("{}/rootfs", pcfg.context_dir);
    let new_root_path = Path::new(&new_root);

    // set permissions
    let mut new_root_perm = fs::metadata(new_root_path)
        .context("unable to get new root perms")?
        .permissions();
    new_root_perm.set_mode(0o777);
    fs::set_permissions(new_root_path, new_root_perm).context("unable to set root permissions")?;

    // unshare
    sched::unshare(CloneFlags::CLONE_NEWUSER | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNET)
        .context("unable to clone newuser")?;

    // notify parent process to do post unshare setup
    sndr.write_all(b"OK").context("error writing to buffer")?;

    // wait for parent process to perform the setup
    let mut buf = [0; 2];

    recv.read_exact(&mut buf[..])
        .context("error reading from buffer")?;

    info!("child - received: {:?}", std::str::from_utf8(&buf).unwrap());

    let (mut c_send, mut p_recv) =
        ipc_unix::unnamed_pipe::pipe(false).context("error creating c->p pipe")?;

    match fork() {
        Ok(Fork::Parent(child)) => {
            unistd::close(c_send).context("unable to close c_send for grandchild")?;

            info!("grandchild has pid: {}", child);
            info!("and now we wait");

            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf[..]).context("error reading")?;

            sndr.write_all(b"OK").context("error writing")?;

            info!("hello from isolated process");

            let c_pid = Pid::from_raw(child);
            match waitpid(c_pid, None) {
                Ok(WaitStatus::Exited(pid, code)) => {
                    info!("child {} exited with status {}", pid, code);
                }
                Ok(status) => {
                    info!("child exited with other status: {:?}", status);
                }
                Err(e) => {
                    error!("waitpid failed: {:?}", e);
                }
            }

            sndr.write_all(b"OK").context("error writing")?;

            Ok(())
        }
        Ok(Fork::Child) => {
            unistd::close(p_recv).context("unable to close c_recv in grandchild")?;

            unistd::setgid(0.into()).context("setgid failed")?;
            unistd::setuid(0.into()).context("setuid failed")?;

            let cf = CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUTS;

            sched::unshare(cf).context("cannot unshare")?;

            setup_mntns(pcfg)?;

            info!("setting up utsns");
            setup_utsns(&String::from("isoproc"))?;
            info!("done setting up utsns");

            c_send
                .write_all(b"OK")
                .context("unable to write to parent")?;

            let path = CString::new("/bin/sh").unwrap();

            let argv = [
                CString::new("sh").unwrap(),
                CString::new("-i").unwrap(), // interactive
            ];

            let env = hashmap_to_cstring_env(&pcfg.env);

            unistd::execve(&path, &argv, &env).context("execve failed")?;

            Ok(())
        }
        Err(err) => {
            bail!("unable to fork {err}")
        }
    }
}

pub fn setup_utsns(hostname: &str) -> Result<()> {
    unistd::sethostname(hostname).context("unable to sethostname")?;

    Ok(())
}

pub fn setup_userns(pid: &i32) -> Result<()> {
    info!("setting up userns");

    let uidmap_path = format!("/proc/{}/uid_map", pid);
    let mut uidmap_file =
        File::create(Path::new(&uidmap_path)).context("unable to open uid_map")?;
    uidmap_file
        .write_all(b"0 1000 1\n")
        .context("unable to write to uid_map")?;

    let setgroups_path = format!("/proc/{}/setgroups", pid);
    let mut setgroups_file =
        File::create(Path::new(&setgroups_path)).context("unable to open setgroups")?;
    setgroups_file
        .write_all(b"deny")
        .context("unable to write to setgroups")?;

    let gidmap_path = format!("/proc/{}/gid_map", pid);
    let mut gidmap_file =
        File::create(Path::new(&gidmap_path)).context("unable to open gid_map")?;
    gidmap_file
        .write_all(b"0 1000 1\n")
        .context("unable to write to gid_map")?;

    info!("done setting up userns");
    Ok(())
}

fn setup_procfs() -> Result<()> {
    info!("setting up procfs");

    let proc_path = Path::new("/proc");

    info!("removing old proc dir");
    if proc_path.exists() {
        fs::remove_dir(proc_path).context("unable to remove /proc")?;
    }
    fs::create_dir(proc_path).context("unable to create proc")?;

    info!("updating proc permissions");
    let mut proc_perm = fs::metadata(proc_path)
        .context("unable to get permissions")?
        .permissions();
    proc_perm.set_mode(0o777);
    fs::set_permissions(proc_path, proc_perm).context("unable to set proc permissions")?;

    info!("mounting as proc");

    mount::<_, _, _, _>(
        Some("proc"),
        proc_path,
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    )
    .context("unable to mount proc")?;

    Ok(())
}

pub fn setup_mntns(pcfg: &ProcessConfig) -> Result<()> {
    let new_root = format!("{}/rootfs", pcfg.context_dir);
    let put_old = format!("{}/.put_old", new_root);

    let new_root_path = Path::new(&new_root);
    let put_old_path = Path::new(&put_old);

    // ensure no shared propagation
    info!("ensuring no shared propagation");
    let msflags = MsFlags::MS_REC | MsFlags::MS_PRIVATE;
    mount::<_, _, _, _>(
        None::<&Path>,
        Path::new("/"),
        None::<&str>,
        msflags,
        None::<&str>,
    )
    .context("error ensuring no shared propagation")?;

    // ensure new root is a mount point
    info!("ensuring new root is a mount point");
    let fstype = Some("ext4");
    mount::<_, _, _, _>(
        Some(new_root_path),
        new_root_path,
        fstype,
        MsFlags::MS_BIND,
        None::<&str>,
    )
    .context("error ms_bind")?;

    // because I create put_old on the same path multiple times while testing
    if put_old_path.exists() {
        info!("put_old exists - removing it");
        fs::remove_dir(put_old_path).context("unable to remove previous .put_old")?;
    }

    info!("creating new put_old");
    fs::create_dir(put_old_path).context("unable to create new put_old")?;
    let mut put_old_perm = fs::metadata(put_old_path)
        .context("unable to get permissions")?
        .permissions();
    put_old_perm.set_mode(0o777);
    fs::set_permissions(put_old_path, put_old_perm).context("unable to set permissions")?;

    // pivot root
    info!("pivoting root");
    unistd::pivot_root(new_root_path, put_old_path).context("unable to pivot root")?;

    info!("changing dir to root");
    unistd::chdir("/").context("unable to chdir to new root")?;

    setup_procfs()?;

    info!("unmounting put_old");
    let isoproc_put_old = "/.put_old";
    let isoproc_put_old_path = Path::new(isoproc_put_old);
    umount2(isoproc_put_old_path, MntFlags::MNT_DETACH).context("unable to umount2 put_old")?;

    info!("removing put_old");
    fs::remove_dir(isoproc_put_old_path).context("unable to rmdir put_old")?;

    Ok(())
}
