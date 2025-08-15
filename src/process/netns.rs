use log::{info, error};
use netlink_packet_route::link::{LinkMessage, LinkAttribute};
use rtnetlink::{self, LinkVeth, Handle};
use std::net::{IpAddr, Ipv4Addr};
use futures::stream::TryStreamExt;
use std::error::Error;


pub fn get_netlink_handle() -> Result<Handle, Box<dyn Error>> {
    match rtnetlink::new_connection() {
        Ok((conn, handle, _)) => {
            tokio::spawn(conn);
            Ok(handle)
        }
        Err(e) => {
            error!("Failed to create netlink connection: {e}");
            Err(Box::new(e))
        }
    }
}


pub async fn create_veth_pair(handle: &Handle) -> Result<(), Box<dyn Error>>{

    info!("creating veth pair");
  
    handle
        .link()
        .add(LinkVeth::new("veth1", "veth1-peer").build())
        .execute()
        .await?; 

    info!("added veth pair");

    Ok(())

}


pub async fn set_root_veth_ip(handle: &Handle) -> Result<(), Box<dyn Error>> {

    info!("setting root veth ip"); 
    let mut links = handle
        .link()
        .get()
        .match_name("veth1".into())
        .execute(); 

    if let Some(link) = links.try_next().await? {
        handle
            .address()
            .add(link.header.index, IpAddr::V4(Ipv4Addr::new(10, 1, 1, 1)), 24)
            .execute()
            .await?;
    }

    info!("done setting root veth IP");
    Ok(())
}


pub async fn set_ns_veth_ip(handle: &Handle) -> Result<(), Box<dyn Error>>{

    info!("setting ns veth ip"); 
    let mut links = handle
        .link()
        .get()
        .match_name("veth1-peer".into())
        .execute(); 

    if let Some(link) = links.try_next().await? {
        handle
            .address()
            .add(link.header.index, IpAddr::V4(Ipv4Addr::new(10, 1, 1, 2)), 24)
            .execute()
            .await?;
    }
    info!("done setting root veth IP");

    Ok(())

}


pub async fn move_veth_to_netns(handle: &Handle, veth_name: &String, ns_fd: &i32) -> Result<(), Box<dyn Error>>{


    info!("moving veth to netns");

    let mut links = handle
        .link()
        .get()
        .match_name("veth1-peer".into())
        .execute(); 

    if let Some(link) = links.try_next().await? {

        let attributes = vec![
            LinkAttribute::NetNsFd(*ns_fd),
            LinkAttribute::IfName(veth_name.into())          
        ];

        let header = link.header.clone();

        let mut link_msg: LinkMessage = Default::default();
        link_msg.header = header;
        link_msg.attributes = attributes;

        handle
            .link()
            .set(link_msg)
            .execute()
            .await?;

        info!("done moving veth to ns by pid");
        return Ok(())
    }

    info!("unable to move veth to ns by pid");

    Ok(())
}
