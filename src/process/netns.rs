use log::info;
use netlink_packet_route::link::{LinkMessage, LinkAttribute};
use rtnetlink::{self, LinkVeth, Handle};
use std::net::{IpAddr, Ipv4Addr};
use futures::stream::TryStreamExt;


pub fn get_netlink_handle() -> Handle {
    let (conn, handle, _) = rtnetlink::new_connection().expect("unable to create new netlink connection");
    tokio::spawn(conn);

    handle
}


pub async fn create_veth_pair(handle: &Handle) {

    info!("creating veth pair");
  
    info!("adding veth pair");
    handle
        .link()
        .add(LinkVeth::new("veth1", "veth1-peer").build())
        .execute()
        .await
        .expect("unable to create veth pair"); 

}


pub async fn set_root_veth_ip(handle: &Handle) {

    info!("setting root veth ip"); 
    let mut links = handle
        .link()
        .get()
        .match_name("veth1".into())
        .execute(); 

    if let Some(link) = links.try_next().await.expect("unable retrieve link by name") {
        handle
            .address()
            .add(link.header.index, IpAddr::V4(Ipv4Addr::new(10, 1, 1, 1)), 24)
            .execute()
            .await
            .expect("unable to add IP address");
    }

    info!("done setting root veth IP");
}


pub async fn set_ns_veth_ip(handle: &Handle) {

    info!("setting root veth ip"); 
    let mut links = handle
        .link()
        .get()
        .match_name("veth1".into())
        .execute(); 

    if let Some(link) = links.try_next().await.expect("unable retrieve link by name") {
        handle
            .address()
            .add(link.header.index, IpAddr::V4(Ipv4Addr::new(10, 1, 1, 2)), 24)
            .execute()
            .await
            .expect("unable to add IP address");
    }
    info!("done setting root veth IP");

}


pub async fn move_veth_to_netns(handle: &Handle, veth_name: &String, ns_fd: &i32) {


    info!("moving veth to netns");

    let mut links = handle
        .link()
        .get()
        .match_name("veth1-peer".into())
        .execute(); 
    if let Some(link) = links.try_next().await.expect("unable retrieve link by name") {

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
            .await
            .expect("unable to move veth to ns by PID");

        info!("done moving veth to ns by pid");
        return
    }

    info!("unable to move veth to ns by pid");

}
