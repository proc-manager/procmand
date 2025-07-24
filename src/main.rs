mod common;
mod process;

use std::io::{Read, Write};

use env_logger::Builder;
use log::{LevelFilter, info};

use fork::{Fork, fork};
use interprocess::os::unix as ipc_unix;
use nix::unistd;
use process::parser;

use common::models::ProcessConfig;
use process::{isoproc, netns};

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
async fn start_process(pcfg: ProcessConfig) {
    let (mut p_send, mut c_recv) =
        ipc_unix::unnamed_pipe::pipe(false).expect("error creating p->c pipe");
    let (mut c_send, mut p_recv) =
        ipc_unix::unnamed_pipe::pipe(false).expect("error creating c->p pipe");

    match fork() {
        Ok(Fork::Parent(child)) => {
            info!("continuing in parent process: {}", child);

            // close child fds
            unistd::close(c_recv).expect("unable to clone c_recv");
            unistd::close(c_send).expect("unable to clone c_send");

            // wait for child process to unshare
            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf).expect("parent: error reading");
            info!(
                "parent - received: {:?}",
                std::str::from_utf8(&buf).unwrap()
            );


            netns::create_veth_pair().await;

            isoproc::setup_userns(&child);

            p_send.write_all(b"OK").expect("parent: error writing");

            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf).expect("parent: error reading");
            info!(
                "parent - received: {:?}",
                std::str::from_utf8(&buf).unwrap()
            );

            info!("child process setup done: waiting to exit");

            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf).expect("parent: error reading");
            info!(
                "parent - received: {:?}",
                std::str::from_utf8(&buf).unwrap()
            );
        }
        Ok(Fork::Child) => {
            info!("in child process");
            unistd::close(p_recv).expect("unable to close p_recv");
            unistd::close(p_send).expect("unable to close p_send");
            isoproc::setup_isoproc(&pcfg, &mut c_recv, &mut c_send);
        }
        Err(_) => {
            info!("fork failed");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    info!("reading the json config");

    let config = parser::parse("process.json")?;
    start_process(config).await;

    info!("done reading json config");

    Ok(())
}
