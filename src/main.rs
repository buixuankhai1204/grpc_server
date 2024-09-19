use std::future::Future;

use tonic::transport::Server;

use crate::live_connection::connection_server::ConnectionServer;
use crate::server::{first_time_grpc_online, LiveConnection};
use crate::storage::Config;

pub mod server;

pub mod live_connection;
mod storage;

mod live_connection_proto {
    include!("live_connection.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let current_address = "127.0.0.1:3000".parse()?;
    // let backup_address = "127.0.0.1:3001".parse()?;

    let addr = "127.0.0.1:9002";
    let mut live_connection = LiveConnection::default();
    live_connection.user_online.insert("xuankhai".to_string(), "ip_1".to_string());
    live_connection.topic_location.insert("topic_1".to_string(), "ip_2".to_string());
    live_connection.topics_by_ip.insert("ip_1".to_string(), vec!["topic_1".to_string(), "topic_2".to_string()]);
    // let mut backup_add: Vec<String> = vec![];
    // let backup_ip = Config::get_all_ip_backup_server().await;
    // match backup_ip {
    //     Ok(value) => {
    //         for config in value.into_iter() {
    //             if config.server_ip != addr {
    //                 backup_add.push(config.server_ip);
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         panic!("{}", error);
    //     }
    // }
    // live_connection = first_time_grpc_online(backup_add[0].to_string())
    //     .await?
    //     .into_inner();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(live_connection_proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(ConnectionServer::new(live_connection))
        .add_service(reflection_service)
        .serve(addr.parse()?)
        .await?;
    Ok(())
}


