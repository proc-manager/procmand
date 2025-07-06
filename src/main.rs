mod process;
mod common;

use log::{info, LevelFilter};
use env_logger::Builder;
use process::parser;

use common::models::ProcessConfig;
use common::constants::{STACK_SIZE};

use nix::sched::CloneFlags;
use fork::{daemon, Fork};


/*

    Responsible for:
        1. reading from the unix socket for new thread requests
        2. calling isoproc for instantiating new isolated processes

*/


#[allow(dead_code)]
fn start_process(pcfg: ProcessConfig) {
    println!("{:?}", pcfg);
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    info!("reading the json config");

    let config = parser::parse("process.json")?;

    if let Ok(Fork::Child) = daemon(false, false) {
        start_process(config.clone());
    }
    

    println!("config: {:?}", config);

    info!("done reading json config");

    Ok(())
}
