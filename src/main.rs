mod process;
mod common;

use log::{info, LevelFilter};
use env_logger::Builder;
use process::parser;

use common::models::ProcessConfig;
use common::constants::{STACK_SIZE};

use nix::sched::CloneFlags;
use fork::{fork, Fork};


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

    match fork() {
        Ok(Fork::Parent(child)) => {
            println!("continuing execution in parent process, new child pid: {}", child);
        }
        Ok(Fork::Child) => {
            start_process(config.clone());
        }
        Err(_) => println!("fork failed")
    }

    info!("done reading json config");

    Ok(())
}
