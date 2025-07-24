use rtnetlink::{self, LinkVeth, Handle};
use std::net::{IpAddr, Ipv4Addr};
use futures::stream::TryStreamExt;


pub fn get_netlink_handle() -> Handle {
    let (conn, handle, _) = rtnetlink::new_connection().expect("unable to create new netlink connection");
    tokio::spawn(conn);

    handle
}

pub async fn create_veth_pair() {


    let handle = get_netlink_handle();

    handle
        .link()
        .add(LinkVeth::new("veth1", "veth1-peer").build())
        .execute()
        .await
        .expect("unable to create veth pair");

    let mut links = handle
        .link()
        .get()
        .match_name("veth1".into())
        .execute(); 

    if let Some(link) = links.try_next().await.expect("unable retrieve link by name") {
        handle
            .address()
            .add(
                link.header.index, IpAddr::V4(Ipv4Addr::new(192, 168, 100, 1)), 24
                )
            .execute()
            .await
            .expect("unable to add IP address");
    }
 
}




