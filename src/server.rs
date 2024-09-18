use std::collections::{HashSet};
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

#[derive(Debug)]
pub struct LiveConnection {
    user_online: DashMap<String, String>,
    topic_location: DashMap<String, String>,
    topics_by_ip: HashSet<String, Vec<String>>,
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
        Ok(Response::new(IpTopicInitResponse {
            topic_id: "asfsafsdf".to_string(),
            ip: "sfasfasf".to_string(),
        }))
    }
}

// -----------------------------------------------------------------------------
// Testing
// -----------------------------------------------------------------------------


