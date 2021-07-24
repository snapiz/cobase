use actix::{Actor, Context, Message as ActixMessage};
use couchbase::{Bucket, Cluster};
use eventstore::Client;
use std::sync::Arc;

use super::error::Error;

pub struct Bus {
    pub eventstore_client: Client,
    pub couchbase_cluster: Arc<Cluster>,
    pub couchbase_bucket: Arc<Bucket>,
}

impl Actor for Bus {
    type Context = Context<Self>;
}

#[derive(ActixMessage)]
#[rtype(result = "Result<(), Error>")]
pub struct Message<T>(pub String, pub T);
