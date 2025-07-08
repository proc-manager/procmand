use crate::common::models::ProcessConfig;

use std::path;

use log::info;
use nix::{sched::{self, CloneFlags}, unistd};

pub fn setup_isoproc(pcfg: &ProcessConfig) {
    
    info!("setting up the isolated process");

    unistd::chdir(path::Path::new(&pcfg.context_dir)).expect("unable to chdir");
    let cf = CloneFlags::CLONE_NEWNS;
    sched::unshare(cf).expect("unable to unshare");
    unistd::chroot(path::Path::new(&pcfg.context_dir)).expect("unable to chroot");
    println!("hello from chroot process");    

}
