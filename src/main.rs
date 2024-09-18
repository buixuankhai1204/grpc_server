use tonic::transport::Server;
use crate::live_connection::connection_server::ConnectionServer;
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
    let addr = "127.0.0.1:9001".parse()?;
    let live_connection = LiveConnection::default();

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
