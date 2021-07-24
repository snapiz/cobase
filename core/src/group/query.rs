use actix::{Actor, Addr, AsyncContext, Context, Handler, Message, ResponseFuture};
use async_trait::async_trait;
use couchbase::{Bucket, Cluster, QueryOptions, UpsertOptions};
use eventstore::Client;
use futures::executor::block_on_stream;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};

use crate::bus::Bus;
use crate::error::Error;
use crate::event::{Domain, EventActor};
use crate::publisher::{PubMessage, Publisher};

use super::event::{Created, Event};

#[derive(Deserialize, Serialize)]
pub struct Group {
    pub r#type: String,
    pub id: String,
    pub user_id: String,
    pub name: String,
}

#[derive(Clone)]
pub struct EventHandler {
    pub eventstore_client: Client,
    pub couchbase_cluster: Arc<Cluster>,
    pub couchbase_bucket: Arc<Bucket>,
    pub publisher: Addr<Publisher>,
}

impl Actor for EventHandler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.clone().start_subscribe(
            ctx.address(),
            self.eventstore_client.to_owned(),
            "$ce-cobase:group",
            "cobase-api-query",
        );
    }
}

#[async_trait]
impl EventActor for EventHandler {
    type Actor = Self;

    async fn handle<'a>(
        &self,
        addr: actix::Addr<Self>,
        event: &'a eventstore::RecordedEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let group_event = Event::from_str(&event.event_type)?;

        match group_event {
            Event::Created => {
                let event_data = event.as_json::<Created>()?;
                addr.send(CreatedEventMessage(event.stream_id.to_owned(), event_data))
                    .await??;
            }
        };

        Ok(())
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), Error>")]
pub struct CreatedEventMessage(String, Created);

impl Handler<CreatedEventMessage> for EventHandler {
    type Result = ResponseFuture<Result<(), Error>>;

    fn handle(&mut self, msg: CreatedEventMessage, _ctx: &mut Self::Context) -> Self::Result {
        let bucket = self.couchbase_bucket.clone();
        let publisher = self.publisher.clone();

        Box::pin(async move {
            let id = Domain::Group.from_stream_id(msg.0.to_owned());
            let user_id = msg.1.user_id.to_owned();

            let content = Group {
                r#type: Domain::Group.document_type(),
                id,
                user_id: msg.1.user_id,
                name: msg.1.name,
            };

            let data = serde_json::to_value(&content)?;

            bucket
                .default_collection()
                .upsert(msg.0, content, UpsertOptions::default())
                .await?;

            publisher.do_send(PubMessage {
                topic: Domain::Group.as_ref().to_owned(),
                user_id: Some(user_id),
                event_type: Event::Created.as_ref().to_owned(),
                data,
            });

            Ok(())
        })
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Group>, Error>")]
pub struct FindGroupsQueryMessage(pub String);

impl Handler<FindGroupsQueryMessage> for Bus {
    type Result = ResponseFuture<Result<Vec<Group>, Error>>;

    fn handle(&mut self, msg: FindGroupsQueryMessage, _ctx: &mut Self::Context) -> Self::Result {
        let cluster = self.couchbase_cluster.clone();
        let bucket = self.couchbase_bucket.clone();

        Box::pin(async move {
            let query = format!(
                "select * from `{}` where type = '{}' and user_id = '{}'",
                bucket.name(),
                Domain::Group.document_type(),
                msg.0
            );

            let mut result = cluster.query(query, QueryOptions::default()).await?;
            let mut rows = Vec::new();

            for row in block_on_stream(result.rows::<serde_json::Value>()) {
                let row = row.map_err(|e| e.to_string())?;
                let row = row
                    .get(bucket.name())
                    .ok_or("Bucket not found ::FindGroupsQueryMessage")?;

                rows.push(serde_json::from_value(row.clone())?);
            }

            Ok(rows)
        })
    }
}
