use charybdis::macros::charybdis_model;
use charybdis::operations::{Find, Insert};
use charybdis::types::{Text, Uuid};
use scylla::{CachingSession, Session, SessionBuilder};
use serde::{Deserialize, Serialize};
use anyhow::Result as AnyhowResult;
use anyhow::Error as AnhowError;

pub async fn create_session() -> Session {
    SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .user("admin", "admin")
        .keyspaces_to_fetch(["uploadasset".to_string()])
        .build()
        .await
        .expect("Failed to create session")
}

#[charybdis_model(
    table_name = uptop.config,
    partition_keys = [server_type],
    clustering_keys = [server_ip],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = [],
)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Config {
    pub server_type: Text,
    pub server_ip: Text,
    pub server_domain: Text,
}

impl Config {
    pub async fn get_all_ip_backup_server() -> AnyhowResult<Vec<Config>> {
        let session: &CachingSession = &CachingSession::from(create_session().await, 1);
        // let config = Config {
        //     server_type: "grpc_server_2".to_string(),
        //     server_ip: "127.0.0.1:90002".to_string(),
        //     server_domain: "empty".to_string(),
        // };
        // config.insert().execute(session).await?;
        let config = Self::find_by_server_type("grpc_server".to_string()).execute(session).await?;
        match config.try_collect().await {
            Ok(value) => {
                Ok(value)
            }
            Err(error) => {
                Err(AnhowError::from(error))
            }
        }
    }
}