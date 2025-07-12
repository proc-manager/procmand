use crate::common::models::ProcessConfig;

use std::{io::{Read, Write}, os::unix::fs::PermissionsExt, path::Path};
use std::fs::{self, File, read_to_string};

use log::{info, error};

use fork::{fork, Fork};
use nix::{mount::{mount, umount2, MntFlags, MsFlags}, sched::{self, CloneFlags}, unistd};
use interprocess::unnamed_pipe::{Sender, Recver};
use interprocess::os::unix as ipc_unix;


pub fn setup_isoproc(pcfg: &ProcessConfig, recv: &mut Recver, sndr: &mut Sender) {
    
    info!("setting up the isolated process");

    // unshare 
    sched::unshare(CloneFlags::CLONE_NEWUSER | CloneFlags::CLONE_NEWPID).expect("unable to clone newuser");

    // notify parent process to do post unshare setup
    sndr.write_all(b"OK").expect("error writing");

    // wait for parent process to perform the setup
    let mut buf = [0; 2];
    recv.read_exact(&mut buf[..]).expect("error reading");
    info!("child - received: {:?}", std::str::from_utf8(&buf).unwrap());

    let (mut c_send, mut p_recv) = ipc_unix::unnamed_pipe::pipe(false)
        .expect("error creating c->p pipe");

    match fork() {
        Ok(Fork::Parent(child)) => {
            unistd::close(c_send).expect("unable to close c_send for grandchild");

            // Display contents
            //
            let uidmap_path = format!("/proc/{}/uid_map", child);
            let gidmap_path = format!("/proc/{}/gid_map", child);
            let setgroups_path = format!("/proc/{}/setgroups", child);
            let uid_map = read_to_string(&uidmap_path).unwrap_or_else(|e| format!("Error reading uid_map: {}", e));
            let gid_map = read_to_string(&gidmap_path).unwrap_or_else(|e| format!("Error reading gid_map: {}", e));
            let setgroups = read_to_string(&setgroups_path).unwrap_or_else(|e| format!("Error reading setgroups: {}", e));

            info!("uid_map:\n{}", uid_map.trim());
            info!("gid_map:\n{}", gid_map.trim());
            info!("setgroups:\n{}", setgroups.trim());

            info!("grandchild has pid: {}", child);
            info!("and now we wait");
            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf[..]).expect("error reading");
            sndr.write_all(b"OK").expect("error writing");
            
            info!("hello from isolated process");    
        },
        Ok(Fork::Child) => {
            unistd::close(p_recv).expect("unable to close c_recv in grandchild");

            unistd::setgid(0.into()).expect("setgid failed");
            unistd::setuid(0.into()).expect("setuid failed");

            let cf = CloneFlags::CLONE_NEWNS 
                | CloneFlags::CLONE_NEWUTS 
                | CloneFlags::CLONE_NEWNET;

            sched::unshare(cf).expect("cannot unshare");

            setup_mntns(pcfg);

            info!("setting up utsns");
            setup_utsns();
            info!("done setting up utsns");
            c_send.write_all(b"OK").expect("unable to send OK from grandshild");
        }, 
        Err(err) => {
            error!("unable to fork under child process: {}", err);
        }
    }

    sndr.write_all(b"OK").expect("error writing");

}


pub fn setup_utsns() {
    unistd::sethostname("isoproc").expect("unable to sethostname");
}


pub fn setup_userns(pid: &i32) {
    info!("setting up userns");

    // Write UID map
    let uidmap_path = format!("/proc/{}/uid_map", pid);
    let mut uidmap_file = File::create(Path::new(&uidmap_path))
        .expect("unable to open uid_map");
    uidmap_file.write_all(b"0 1000 1\n")
        .expect("unable to write to uid_map");

    // Must deny setgroups *before* writing gid_map
    let setgroups_path = format!("/proc/{}/setgroups", pid);
    let mut setgroups_file = File::create(Path::new(&setgroups_path))
        .expect("unable to open setgroups");
    setgroups_file.write_all(b"deny")
        .expect("unable to write to setgroups");

    // Write GID map
    let gidmap_path = format!("/proc/{}/gid_map", pid);
    let mut gidmap_file = File::create(Path::new(&gidmap_path))
        .expect("unable to open gid_map");
    gidmap_file.write_all(b"0 1000 1\n")
        .expect("unable to write to gid_map");

    info!("done setting up userns");

    // Display contents
    //
    //d
    let uid_map = read_to_string(&uidmap_path).unwrap_or_else(|e| format!("Error reading uid_map: {}", e));
    let gid_map = read_to_string(&gidmap_path).unwrap_or_else(|e| format!("Error reading gid_map: {}", e));
    let setgroups = read_to_string(&setgroups_path).unwrap_or_else(|e| format!("Error reading setgroups: {}", e));

    info!("uid_map:\n{}", uid_map.trim());
    info!("gid_map:\n{}", gid_map.trim());
    info!("setgroups:\n{}", setgroups.trim());

    info!("done setting up userns");
}


fn setup_procfs() {

    info!("setting up procfs");

    let proc_path = Path::new("/proc");
    
    info!("removing old proc dir");
    if proc_path.exists() {
        fs::remove_dir(proc_path).expect("unable to remove /proc");
    }
    fs::create_dir(proc_path).expect("unable to create proc");
    
    info!("updating proc permissions");
    let mut proc_perm = fs::metadata(proc_path).expect("unable to get permissions").permissions();
    proc_perm.set_mode(0o777);
    fs::set_permissions(proc_path, proc_perm).expect("unable to set proc permissions");

    println!("euid: {}", unistd::geteuid());
    println!("guid: {}", unistd::getgid());
    println!("uid: {}", unistd::getuid());
    println!("procpath: {:?}", proc_path);

    info!("mounting as proc");
        
    mount::<_, _, _, _>(
        Some("proc"),
        proc_path,
        Some("proc"), 
        MsFlags::empty(),
        None::<&str>
    ).expect("unable to mount proc");

}


pub fn setup_mntns(pcfg: &ProcessConfig) {
    

    println!("euid: {}", unistd::geteuid());
    println!("guid: {}", unistd::getgid());
    println!("uid: {}", unistd::getuid());

    let new_root = format!("{}/rootfs", pcfg.context_dir);    
    let put_old  = format!("{}/.put_old", new_root);

    let new_root_path = Path::new(&new_root);
    let put_old_path  = Path::new(&put_old);

    let mut new_root_perm = fs::metadata(new_root_path).expect("unable to get new root perms").permissions();
    new_root_perm.set_mode(0o777);
    fs::set_permissions(new_root_path, new_root_perm).expect("unable to set root permissions");

    // ensure no shared propagation
    info!("ensuring no shared propagation");
    let msflags = MsFlags::MS_REC | MsFlags::MS_PRIVATE;
    mount::<_, _, _, _>(
        None::<&Path>, 
        Path::new("/"),
        None::<&str>, 
        msflags, None::<&str>
    ).expect("error ensuring no shared propagation");


    // ensure new root is a mount point
    info!("ensuring new root is a mount point");
    let fstype = Some("ext4");
    mount::<_, _, _, _>(
        Some(new_root_path),
        new_root_path,
        fstype,
        MsFlags::MS_BIND,
        None::<&str>,
    ).expect("error ms_bind");

    // because I create put_old on the same path multiple times while testing
    if put_old_path.exists() {
        info!("put_old exists - removing it");
        fs::remove_dir(put_old_path).expect("unable to remove previous .put_old");
    }

    info!("creating new put_old");
    fs::create_dir(put_old_path).expect("unable to create new put_old");
    let mut put_old_perm = fs::metadata(put_old_path).expect("unable to get permissions").permissions();
    put_old_perm.set_mode(0o777);
    fs::set_permissions(put_old_path, put_old_perm).expect("unable to set permissions");


    // pivot root
    info!("pivoting root");
    unistd::pivot_root(new_root_path, put_old_path).expect("unable to pivot root");

    info!("changing dir to root");
    unistd::chdir("/").expect("unable to chdir to new root");

    setup_procfs();

    info!("unmounting put_old");
    let isoproc_put_old = "/.put_old";
    let isoproc_put_old_path = Path::new(isoproc_put_old);
    umount2(isoproc_put_old_path, MntFlags::MNT_DETACH).expect("unable to umount2 put_old");

    info!("removing put_old");
    fs::remove_dir(isoproc_put_old_path).expect("unable to rmdir put_old");

}



