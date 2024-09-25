use std::cell::{Cell, RefCell};
use std::future::Future;
use std::net::ToSocketAddrs;
use tonic::transport::Server;

use crate::live_connection::connection_server::ConnectionServer;
use crate::server::{first_time_grpc_online, LiveConnection};
use crate::http::MessageService;
use crate::message::message_server::MessageServer;
use crate::storage::Config;

pub mod server;

pub mod live_connection;
mod storage;
mod http;
mod message;

mod live_connection_proto {
    include!("live_connection.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}

mod message_proto {
    include!("message.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let current_address = "127.0.0.1:3000".parse()?;
    // let backup_address = "127.0.0.1:3001".parse()?;

    let addr = "127.0.0.1:9001";
    let msg_service = MessageService::new();
    let msg_server = MessageServer::new(msg_service);
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(live_connection_proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(msg_server)
        .add_service(reflection_service)
        .serve(addr.to_socket_addrs().unwrap().next().unwrap())
        .await?;
    Ok(())
}

#[derive(Debug)]
struct Node<'a> {
    val: RefCell<String>,
    adjacent: Vec<&'a Node<'a>>,
}

fn add_one(node: &Node) {
    let mut current_val = node.val.borrow_mut();
    current_val.push_str("asfasthhrthrthrjrtjf");
    for adj in node.adjacent.iter() {
        add_one(*adj);
    }
}
