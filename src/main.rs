mod process;
mod common;

use std::path;
use env_logger::Builder;
use log::{info, LevelFilter};

use nix::sched::{self, CloneFlags};
use fork::{fork, Fork};
use nix::unistd;
use process::parser;

use common::models::ProcessConfig;
use process::isoproc;


/*
    Responsible for:
        1. reading from the unix socket for new thread requests
        2. calling isoproc for instantiating new isolated processes

    Workflow:
        The main function is an event loop that is multi-threaded. 
        It calls start_process with the configuration. 
        The start_process function forks and sets up the new process. 
        
*/
#[allow(dead_code)]
fn start_process(pcfg: ProcessConfig) {
    match fork() {
        Ok(Fork::Parent(child)) => {
            println!("continuing in parent process: {}", child);
        },
        Ok(Fork::Child) => {
            isoproc::setup_isoproc(&pcfg); 
        },
        Err(_) => {
            println!("fork failed");
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    info!("reading the json config");

    let config = parser::parse("process.json")?;
    start_process(config);

    info!("done reading json config");

    Ok(())
}
