use std::collections::{HashMap, HashSet};
use dashmap::DashMap;
use tonic::{Request, Response, Status};

use crate::live_connection::connection_server::Connection;
use crate::live_connection::*;

// -----------------------------------------------------------------------------
// Error Messages
// -----------------------------------------------------------------------------

const BAD_PRICE_ERR: &str = "provided PRICE was invalid";
const DUP_PRICE_ERR: &str = "item is already at this price";
const DUP_ITEM_ERR: &str = "item already exists in inventory";
const EMPTY_QUANT_ERR: &str = "invalid quantity of 0 provided";
const EMPTY_SKU_ERR: &str = "provided SKU was empty";
const NO_ID_ERR: &str = "no ID or SKU provided for item";
const NO_ITEM_ERR: &str = "the item requested was not found";
const NO_STOCK_ERR: &str = "no stock provided for item";
const UNSUFF_INV_ERR: &str = "not enough inventory for quantity change";

// -----------------------------------------------------------------------------
// InventoryServer Implementation
// -----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct LiveConnection {
    user_online: DashMap<String, String>,
    topic_location: DashMap<String, String>,
    topics_by_ip: DashMap<String, Vec<String>>,
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
        Ok(Response::new(IpUserOnlineResponse {
            username: "asfsafsdf".to_string(),
            ip: "sfasfasf".to_string(),
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
        Ok(Response::new(LiveConnectionResponse::try_from(self).expect("can not parse!")))
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
