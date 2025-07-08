use crate::common::models::ProcessConfig;

use std::{io::{Read, Write}, path};

use log::info;
use nix::{sched::{self, CloneFlags}, unistd};
use interprocess::unnamed_pipe::{Sender, Recver};


pub fn setup_isoproc(pcfg: &ProcessConfig, recv: &mut Recver, sndr: &mut Sender) {

    
    info!("setting up the isolated process");

    sndr.write_all(String::from("OK").as_bytes()).expect("error writing");
    let mut buf = [0; 2];
    recv.read_exact(&mut buf[..]).expect("error reading");


    unistd::chdir(path::Path::new(&pcfg.context_dir)).expect("unable to chdir");
    let cf = CloneFlags::CLONE_NEWNS;
    sched::unshare(cf).expect("cannot unshare");
    unistd::chroot(path::Path::new(&pcfg.context_dir)).expect("unable to chroot");

    println!("hello from chroot process");    

}

