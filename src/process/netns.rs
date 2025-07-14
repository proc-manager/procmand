use rtnetlink::{self, LinkVeth};

pub async fn create_veth_pair() {


    let (conn, handle, _) = rtnetlink::new_connection().expect("unable to create new netlink connection");
    tokio::spawn(conn);

    handle
        .link()
        .add(LinkVeth::new("veth1", "veth1-peer").build())
        .execute()
        .await
        .expect("unable to create veth pair");

}
