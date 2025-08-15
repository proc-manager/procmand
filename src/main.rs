mod common;
mod process;

use std::io::{Read, Write};
use std::fs::File;
use std::os::fd::{AsFd, AsRawFd};
use std::error::Error;

use env_logger::Builder;
use log::{LevelFilter, info};
use std::time::Duration;

use fork::{Fork, fork};
use interprocess::os::unix as ipc_unix;
use nix::sched::CloneFlags;
use nix::unistd;
use process::parser;

use common::models::ProcessConfig;
use process::{isoproc, netns};
use rtnetlink::LinkUnspec;

/*
    Responsible for:
        1. reading from the unix socket for new thread requests
        2. calling isoproc for instantiating new isolated processes

    Workflow:
        The main function is an event loop that is multi-threaded.
        It calls start_process with the configuration.
        The start_process function forks and sets up the new process.

*/
async fn start_process(pcfg: ProcessConfig) -> Result<(), Box<dyn Error>> {
    let (mut p_send, mut c_recv) = ipc_unix::unnamed_pipe::pipe(false)
                                    .expect("error creating p->c pipe");
    let (mut c_send, mut p_recv) = ipc_unix::unnamed_pipe::pipe(false)
                                    .expect("error creating c->p pipe");

    match fork() {
        Ok(Fork::Parent(child)) => {
            info!("continuing in parent process: {}", child);

            // close child fds
            unistd::close(c_recv).expect("unable to clone c_recv");
            unistd::close(c_send).expect("unable to clone c_send");

            // wait for child process to unshare
            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf).expect("parent: error reading");
            info!("parent - received: {:?}", std::str::from_utf8(&buf).unwrap());

            isoproc::setup_userns(&child);

            let handle = netns::get_netlink_handle()?;

            netns::create_veth_pair(&handle).await?;
            netns::set_root_veth_ip(&handle).await?;


            let self_pid = std::process::id() as i32;

            let child_ns_file = File::open(format!("/proc/{child}/ns/net"))
                .expect("cannot open child's net ns file");
            let parent_ns_file = File::open(format!("/proc/{self_pid}/ns/net"))
                .expect("cannot open child's net ns file");


            let child_ns_fd = child_ns_file.as_fd();
            let child_ns_rawfd = child_ns_file.as_raw_fd();
            let parent_ns_fd = parent_ns_file.as_fd();

            netns::move_veth_to_netns(
                &handle, 
                &String::from("veth1-peer"), 
                &child_ns_rawfd
            ).await?;

            handle
                .link()
                .set(LinkUnspec::new_with_name("veth1").up().build())
                .execute()
                .await?;

            let _ = nix::sched::setns(child_ns_fd, CloneFlags::CLONE_NEWNET);

            info!("waiting 5 sec");
            std::thread::sleep(Duration::from_secs(5));


            {

                let ns_handle = netns::get_netlink_handle()?;

                netns::set_ns_veth_ip(&ns_handle).await?;
                ns_handle
                    .link()
                    .set(LinkUnspec::new_with_name("veth1-peer").up().build())
                    .execute()
                    .await?;
            }

            let _ = nix::sched::setns(parent_ns_fd, CloneFlags::CLONE_NEWNET);

            p_send.write_all(b"OK")?;

            let mut buf = [0; 2];
            p_recv.read_exact(&mut buf)?;
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    info!("reading the json config");

    let config = parser::parse("process.json")?;
    start_process(config).await?;

    info!("done reading json config");

    Ok(())
}
