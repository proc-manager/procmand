use crate::common::models::ProcessConfig;

use std::{io::{Read, Write}, path::{self, Path}};
use std::fs::{self, File};

use log::info;

use nix::{sched::{self, CloneFlags}, unistd, mount::{mount, MsFlags, umount2, MntFlags}};
use interprocess::unnamed_pipe::{Sender, Recver};


pub fn setup_isoproc(pcfg: &ProcessConfig, recv: &mut Recver, sndr: &mut Sender) {
    
    info!("setting up the isolated process");

    // unshare 
    let cf = CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWUSER | CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWUTS;
    sched::unshare(cf).expect("cannot unshare");

    // notify parent process to do post unshare setup
    sndr.write_all(String::from("OK").as_bytes()).expect("error writing");


    // wait for parent process to perform the setup
    let mut buf = [0; 2];
    recv.read_exact(&mut buf[..]).expect("error reading");

    setup_mntns(pcfg);

    info!("setting up utsns");
    setup_utsns();
    info!("done setting up utsns");

    sndr.write_all(String::from("OK").as_bytes()).expect("error writing");
    
    info!("hello from isolated process");    

}


pub fn setup_utsns() {
    unistd::sethostname("isoproc").expect("unable to sethostname");
}


pub fn setup_userns(pid: &i32) { 
    info!("setting up userns");
    let uid = 1000;

    let uidmap_path = format!("/proc/{}/uid_map", pid);
    let write_line = format!("0 {} 1", uid);    
    let mut uidmap_file = File::create(path::Path::new(&uidmap_path)).expect("unable to open um file");

    uidmap_file.write_all(write_line.as_bytes()).expect("unable to write um");


    let setgroups_path = format!("/proc/{}/setgroups", pid);
    let write_line = "deny";
    let mut setgroups_file = File::create(path::Path::new(&setgroups_path)).expect("unable to open sg file");
    setgroups_file.write_all(write_line.as_bytes()).expect("unable to write sg");


    let gidmap_path = format!("/proc/{}/gid_map", pid);
    let write_line = format!("0 {} 1", uid);    
    let mut gidmap_file = File::create(path::Path::new(&gidmap_path)).expect("unable to open gm file");

    gidmap_file.write_all(write_line.as_bytes()).expect("unable to write um");

    info!("done setting up userns");
}

pub fn setup_mntns(pcfg: &ProcessConfig) {
    
    info!("setting up mntns");
    /*
    if unistd::geteuid() != 0.into() {
        info!("the actual uid is: {}", unistd::geteuid());
        panic!("ayo why you not root");
    }
    */

    let rfs_path = format!("{}/rootfs", pcfg.context_dir).to_string();
    let proc_rootfs = Path::new(&rfs_path);
    let fstype = Some("ext4");
    let mflags = MsFlags::MS_REC | MsFlags::MS_PRIVATE;

    // Remount / as private
    info!("remounting as private");
    mount::<_, _, _, _>(None::<&Path>, Path::new("/"), None::<&str>, mflags, None::<&str>)
        .expect("unable to mount");

    // Bind mount rootfs onto itself
    info!("binding mount rootfs onto itself");
    mount::<_, _, _, _>(
        Some(proc_rootfs),
        proc_rootfs,
        fstype,
        MsFlags::MS_BIND,
        Some(""),
    ).expect("error ms_bind");

    unistd::chdir(proc_rootfs)
        .expect("unable to chdir");

    let put_old = format!("{}/.put_old", rfs_path);
    fs::create_dir_all(&put_old)
        .expect("unable to create .put_old");

    info!("pivoting root");
    unistd::pivot_root(Path::new(&rfs_path), Path::new(&put_old))
        .expect("unable to pivot root");

    info!("chdir to root");
    unistd::chdir("/")
        .expect("unable to chdir to root");

    info!("preparing procfs");
    setup_procfs();

    info!("unmounting .put_old");
    umount2(".put_old", MntFlags::MNT_DETACH).expect("unable to umount");

    info!("removing .put_old");
    fs::remove_dir(Path::new(".put_old")).expect("unable to remove .put_old");

    info!("done setting up mount namespace")

}


fn setup_procfs() {

    info!("creating procfs");

    fs::create_dir_all("/proc")
        .expect("unable to create dir /proc");
    

    let proc_path = Path::new("proc");
    let root_proc_path = Path::new("/proc");
    mount::<_, Path, _, _>(
        Some(proc_path), 
        root_proc_path, 
        Some(proc_path), 
        MsFlags::empty(),
        None::<&Path>
    ).expect("unable to mount");

    
    info!("created procfs");
}

