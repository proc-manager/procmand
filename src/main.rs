mod process;
mod common;

use std::io::{Read, Write};

use env_logger::Builder;
use log::{info, LevelFilter};

use fork::{fork, Fork};
use process::parser;
use interprocess::os::unix as ipc_unix;
use nix::unistd;

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
    
    let (mut p_send, mut c_recv) = ipc_unix::unnamed_pipe::pipe(false)
        .expect("error creating p->c pipe");
    let (mut c_send, mut p_recv) = ipc_unix::unnamed_pipe::pipe(false)
        .expect("error creating c->p pipe");

    match fork() {
        Ok(Fork::Parent(child)) => {
            println!("continuing in parent process: {}", child);
            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf).expect("parent: error reading");
            p_send.write_all(String::from("OK").as_bytes()).expect("parent: error writing");
            isoproc::setup_userns(&child);
        },
        Ok(Fork::Child) => {
            info!("in child process");
            isoproc::setup_isoproc(&pcfg, &mut c_recv, &mut c_send); 
            unistd::close(p_recv).expect("unable to close p_recv");
            unistd::close(p_send).expect("unable to close p_send");
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
