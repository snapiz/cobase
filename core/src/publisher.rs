use actix::prelude::*;
use actix::{Actor, Context};
use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "Result<(), Error>")]
pub struct PubMessage {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub topic: String,
    pub data: Value,
}

#[derive(Clone)]
pub struct Publisher {
    channel: String,
    redis_pool: Pool<RedisConnectionManager>,
}

impl Publisher {
    pub fn new(channel: String, redis_client: redis::Client) -> Self {
        let redis_manager = RedisConnectionManager::new(redis_client);
        let redis_pool = Pool::builder().max_open(15).build(redis_manager);

        Self {
            redis_pool,
            channel,
        }
    }
}

impl Actor for Publisher {
    type Context = Context<Self>;
}

impl Handler<PubMessage> for Publisher {
    type Result = ResponseFuture<Result<(), Error>>;

    fn handle(&mut self, msg: PubMessage, _: &mut Context<Self>) -> Self::Result {
        let redis_pool = self.redis_pool.clone();
        let channel = self.channel.to_owned();

        Box::pin(async move {
            let mut redis_client = redis_pool.get().await?;

            let message = serde_json::to_string(&msg)?;

            redis_client.publish(channel, message).await?;

            Ok(())
        })
    }
}
