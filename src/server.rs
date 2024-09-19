use crate::live_connection::connection_client::ConnectionClient;
use crate::live_connection::connection_server::{Connection, ConnectionServer};
use crate::live_connection::*;
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Once;
use tonic::transport::{Channel, Server};
use tonic::{Request, Response, Status};
use anyhow::Result as AnyhowResult;
use anyhow::Error as AnyhowError;

#[derive(Debug, Clone)]
pub struct LiveConnection {
    pub user_online: DashMap<String, String>,
    pub topic_location: DashMap<String, String>,
    pub topics_by_ip: DashMap<String, Vec<String>>,
}

impl Default for LiveConnection {
    fn default() -> Self {
        LiveConnection {
            user_online: Default::default(),
            topic_location: Default::default(),
            topics_by_ip: Default::default(),
        }
    }
}


#[tonic::async_trait]
impl Connection for LiveConnection {
    async fn get_ip_user_online(&self, request: Request<UsernameRequest>) -> Result<Response<IpUserOnlineResponse>, Status> {
        Ok(Response::new(IpUserOnlineResponse {
            username: "asfsafsdf".to_string(),
            ip: "sfasfasf".to_string(),
        }))
    }

    async fn add_ip_user_online(&self, request: Request<AddIpForUserRequest>) -> Result<Response<IpUserOnlineResponse>, Status> {
        let request_object = &request.into_inner();
        self.user_online.insert((*request_object.username.to_string()).parse().unwrap(), request_object.ip.to_string());

        Ok(Response::new(IpUserOnlineResponse {
            username: request_object.username.to_string(),
            ip: request_object.ip.to_string(),
        }))
    }

    async fn get_ip_topic_init(&self, request: Request<TopicIdRequest>) -> Result<Response<IpTopicInitResponse>, Status> {
        Ok(Response::new(IpTopicInitResponse {
            topic_id: "asfsafsdf".to_string(),
            ip: "sfasfasf".to_string(),
        }))
    }

    async fn add_ip_for_topic_init(&self, request: Request<AddIpForTopicRequest>) -> Result<Response<IpTopicInitResponse>, Status> {
        let a = request.into_inner();

        Ok(Response::new(IpTopicInitResponse {
            topic_id: "asfsafsdf".to_string(),
            ip: "sfasfasf".to_string(),
        }))
    }

    async fn get_all_topics_id_by_ip(&self, request: Request<IpRequest>) -> Result<Response<AllTopicsByIpResponse>, Status> {
        todo!()
    }

    async fn push_new_topic_to_ip(&self, request: Request<PushNewTopicToIpRequest>) -> Result<Response<PushNewTopicToIpResponse>, Status> {
        todo!()
    }

    async fn pop_old_topic_to_ip(&self, request: Request<PopOldTopicToIpRequest>) -> Result<Response<PopOldTopicToIpResponse>, Status> {
        todo!()
    }

    async fn get_full_information_from_backup_server(&self, request: Request<EmptyParam>) -> Result<Response<LiveConnectionResponse>, Status> {
        Ok(Response::new(LiveConnectionResponse::try_from(self).expect("Can not parse!")))
    }
}

// -----------------------------------------------------------------------------
// Testing
// -----------------------------------------------------------------------------
fn dashmap_to_hashmap<K, V>(dashmap: DashMap<K, V>) -> HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    let mut hashmap = HashMap::new();

    for item in dashmap.into_iter() {
        hashmap.insert(item.0, item.1); // item.0 is the key, item.1 is the value
    }

    hashmap
}

fn hashmap_to_dashmap<K, V>(hashmap: HashMap<K, V>) -> DashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    let dashmap = DashMap::new();

    for (key, value) in hashmap {
        dashmap.insert(key, value);
    }

    dashmap
}

fn dashmap_to_hashmap_with_struct<K, V>(hashset: DashMap<K, Vec<String>>) -> HashMap<K, TopicList>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    let mut hashmap = HashMap::new();

    for (key, value) in hashset {
        let mut topic_list: TopicList = TopicList { topics: value };

        hashmap.insert(key, topic_list);
    }

    hashmap
}

fn hashmap_to_dashmap_with_struct<K, V>(hash_map: HashMap<K, TopicList>) -> DashMap<K, Vec<String>>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    let mut dashmap = DashMap::new();
    let mut topic_list: Vec<String> = vec![];
    for (key, topic_list_value) in hash_map {
        topic_list = topic_list_value.topics;

        dashmap.insert(key, topic_list);
    }

    dashmap
}

impl TryFrom<&LiveConnection> for LiveConnectionResponse {
    type Error = ();

    fn try_from(live_connection: &LiveConnection) -> Result<Self, Self::Error> {
        let mut result = LiveConnectionResponse::default();

        if live_connection.user_online.is_empty() {
            return Err(());
        };
        result.user_online = dashmap_to_hashmap(live_connection.clone().user_online);

        if live_connection.topic_location.is_empty() {
            return Err(());
        }
        result.topic_location = dashmap_to_hashmap(live_connection.clone().topic_location);

        if live_connection.topics_by_ip.is_empty() {
            return Err(());
        }

        result.topics_by_ip = dashmap_to_hashmap_with_struct::<String, Vec<String>>(live_connection.clone().topics_by_ip);
        Ok(result)
    }
}

impl TryFrom<&LiveConnectionResponse> for LiveConnection {
    type Error = ();

    fn try_from(live_connection_response: &LiveConnectionResponse) -> Result<Self, Self::Error> {
        let mut result = LiveConnection::default();

        if live_connection_response.user_online.is_empty() {
            return Err(());
        };
        result.user_online = hashmap_to_dashmap(live_connection_response.clone().user_online);

        if live_connection_response.topic_location.is_empty() {
            return Err(());
        }
        result.topic_location = hashmap_to_dashmap(live_connection_response.clone().topic_location);

        if live_connection_response.topics_by_ip.is_empty() {
            return Err(());
        }

        result.topics_by_ip = hashmap_to_dashmap_with_struct::<String, Vec<String>>(live_connection_response.clone().topics_by_ip);
        Ok(result)
    }
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

pub async fn first_time_grpc_online(
    target_ip_backup: String,
) -> AnyhowResult<Response<LiveConnection>> {
    let mut client = get_client(&target_ip_backup).await;
    match client
        .get_full_information_from_backup_server(Request::new(EmptyParam {}))
        .await
    {
        Ok(value) => {
            Ok(Response::new(LiveConnection::try_from(&value.into_inner()).unwrap()))
        }
        Err(err) => {
            Err(AnyhowError::from(err))
        }
    }
}

