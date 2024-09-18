use std::error::Error;
use std::future::Future;
use std::sync::Once;

use anyhow::Result as AnyhowResult;
use tonic::transport::{Channel, Server};
use tonic::{Request, Response};

use crate::live_connection::connection_server::ConnectionServer;
use crate::live_connection_proto::connection_client::ConnectionClient;
use crate::live_connection_proto::{EmptyParam, LiveConnectionResponse};
use crate::server::LiveConnection;

pub mod server;

pub mod live_connection;

mod live_connection_proto {
    include!("live_connection.rs");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("store_descriptor");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let current_address = "127.0.0.1:3000".parse()?;
    // let backup_address = "127.0.0.1:3001".parse()?;

    let addr = "127.0.0.1:9001".parse()?;
    let mut live_connection = LiveConnection::default();
    live_connection = LiveConnectionResponse::try_into(
        first_time_grpc_online("127.0.0.1:9002".to_string())
            .await?
            .into_inner(),
    )
    .expect("fsfsf");

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(live_connection_proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(ConnectionServer::new(live_connection))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}

static SERVER_INIT: Once = Once::new();
async fn get_client(target_ip_backup: &str) -> ConnectionClient<Channel> {
    SERVER_INIT.call_once(|| {
        tokio::spawn(async {
            let addr = "127.0.0.1:8080".parse().unwrap();
            let inventory = LiveConnection::default();
            Server::builder()
                .add_service(ConnectionServer::new(inventory))
                .serve(addr)
                .await
                .unwrap();
        });
    });

    loop {
        match ConnectionClient::connect(target_ip_backup.to_string()).await {
            Ok(client) => return client,
            Err(_) => println!("waiting for server connection"),
        };
    }
}

async fn first_time_grpc_online(
    target_ip_backup: String,
) -> AnyhowResult<Response<LiveConnectionResponse>> {
    let mut client = get_client(&target_ip_backup).await;
    match client
        .get_full_information_from_backup_server(Request::new(EmptyParam {}))
        .await
    {
        Ok(value) => {
            return Ok(value);
        }
        Err(err) => {
            return Err(Error::from(err));
        }
    }
}
