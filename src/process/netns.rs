use futures::stream::TryStreamExt;
use log::info;
use netlink_packet_route::link::{LinkAttribute, LinkMessage};
use rtnetlink::{self, Handle, LinkVeth};
use std::net::{IpAddr, Ipv4Addr};

use anyhow::{Context, Result, bail};

pub fn get_netlink_handle() -> Result<Handle> {
    let (conn, handle, _) =
        rtnetlink::new_connection().context("unable to create netlink connection")?;

    tokio::spawn(conn);

    Ok(handle)
}

pub async fn create_veth_pair(handle: &Handle) -> Result<()> {
    info!("creating veth pair");

    handle
        .link()
        .add(LinkVeth::new("veth1", "veth1-peer").build())
        .execute()
        .await?;

    info!("added veth pair");

    Ok(())
}

pub async fn set_root_veth_ip(handle: &Handle) -> Result<()> {
    info!("setting root veth ip");
    let mut links = handle.link().get().match_name("veth1".into()).execute();

    match links.try_next().await? {
        Some(link) => {
            handle
                .address()
                .add(
                    link.header.index,
                    IpAddr::V4(Ipv4Addr::new(10, 1, 1, 1)),
                    24,
                )
                .execute()
                .await?;
            info!("done setting root veth IP");

            Ok(())
        }
        None => {
            bail!("error setting root veth IP");
        }
    }
}

pub async fn set_ns_veth_ip(handle: &Handle) -> Result<()> {
    info!("setting ns veth ip");
    let mut links = handle
        .link()
        .get()
        .match_name("veth1-peer".into())
        .execute();

    match links.try_next().await? {
        Some(link) => {
            handle
                .address()
                .add(
                    link.header.index,
                    IpAddr::V4(Ipv4Addr::new(10, 1, 1, 2)),
                    24,
                )
                .execute()
                .await?;
            info!("done setting root veth IP");
            Ok(())
        }
        None => {
            bail!("error setting root IP")
        }
    }
}

pub async fn move_veth_to_netns(handle: &Handle, veth_name: &str, ns_fd: &i32) -> Result<()> {
    info!("moving veth to netns");

    let mut links = handle
        .link()
        .get()
        .match_name(veth_name.to_string())
        .execute();

    match links.try_next().await? {
        Some(link) => {
            let attributes = vec![
                LinkAttribute::NetNsFd(*ns_fd),
                LinkAttribute::IfName(veth_name.into()),
            ];

            let header = link.header.clone();

            let mut link_msg: LinkMessage = Default::default();
            link_msg.header = header;
            link_msg.attributes = attributes;

            handle.link().set(link_msg).execute().await?;

            info!("done moving veth to ns by pid");
            Ok(())
        }
        None => {
            bail!("unable to move veth to ns by pid");
        }
    }
}
