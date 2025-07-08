use crate::common::models::ProcessConfig;
use nix::unistd;

pub fn isoproc(pcfg: ProcessConfig) {

    println!("{:?}", pcfg);

}


pub fn prepare_mntns(pcfg: &ProcessConfig) -> Result<(), Err> {

    if unistd::geteuid() != 0 {

    }

}
