use crate::common::models::ProcessConfig;

use std::{io::{Read, Write}, path};
use std::fs::File;

use env_logger::Builder;
use log::{info, LevelFilter};

use nix::{sched::{self, CloneFlags}, unistd};
use interprocess::unnamed_pipe::{Sender, Recver};


pub fn setup_isoproc(pcfg: &ProcessConfig, recv: &mut Recver, sndr: &mut Sender) {
    
    println!("setting up the isolated process");

    sndr.write_all(String::from("OK").as_bytes()).expect("error writing");
    let mut buf = [0; 2];
    recv.read_exact(&mut buf[..]).expect("error reading");

    println!("setting up utsns");
    setup_utsns();
    println!("done setting up utsns");

    unistd::chdir(path::Path::new(&pcfg.context_dir)).expect("unable to chdir");
    let cf = CloneFlags::CLONE_NEWNS;
    sched::unshare(cf).expect("cannot unshare");
    unistd::chroot(path::Path::new(&pcfg.context_dir)).expect("unable to chroot");

    println!("hello from chroot process");    

}


pub fn setup_utsns() {
    unistd::sethostname("isoproc").expect("unable to sethostname");
}


pub fn setup_userns(pid: &i32) { 
    println!("setting up userns");
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

    println!("done setting up userns");
}

