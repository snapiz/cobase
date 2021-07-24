use actix::{Actor, Addr};
use async_trait::async_trait;
use eventstore::{Client, RecordedEvent, SubEvent};
use futures::{future::BoxFuture, FutureExt};
use nanoid::nanoid;
use serde::Serialize;
use std::{thread, time::Duration};
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

#[derive(Debug, Serialize, PartialEq, EnumString, AsRefStr, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Domain {
    Group,
}

impl Domain {
    pub fn to_stream_id(&self) -> String {
        format!("cobase:{}-{}", self.as_ref(), nanoid!())
    }

    pub fn from_stream_id<S: AsRef<str>>(&self, id: S) -> String {
        let from = format!("cobase:{}-", self.as_ref());
        id.as_ref().replace(from.as_str(), "")
    }

    pub fn document_type(&self) -> String {
        format!("cobase:{}", self.as_ref())
    }
}

#[async_trait]
pub trait EventActor: Actor + Send + Sync {
    type Actor: Actor;

    async fn handle<'a>(
        &self,
        addr: Addr<Self::Actor>,
        event: &'a RecordedEvent,
    ) -> Result<(), Box<dyn std::error::Error>>;

    fn start_subscribe<S: AsRef<str> + Sync + Send + 'static>(
        self,
        addr: Addr<Self::Actor>,
        client: Client,
        stream: S,
        group_name: S,
    ) {
        tokio::spawn(async move {
            self.subscribe_with_retry::<_>(addr, client, stream, group_name)
                .await
        });
    }

    fn subscribe_with_retry<S: AsRef<str> + Sync + Send + 'static>(
        &self,
        addr: Addr<Self::Actor>,
        client: Client,
        stream: S,
        group_name: S,
    ) -> BoxFuture<'_, ()> {
        async move {
            if let Err(err) = self
                .subscribe::<_>(
                    addr.to_owned(),
                    client.to_owned(),
                    stream.as_ref(),
                    group_name.as_ref(),
                )
                .await
            {
                error!("{}", err);
            }

            thread::sleep(Duration::from_secs(2));

            info!(
                "try to subscribe {}::{}",
                group_name.as_ref(),
                stream.as_ref()
            );

            self.subscribe_with_retry::<_>(addr, client, stream, group_name)
                .await;
        }
        .boxed()
    }

    async fn subscribe<S: AsRef<str> + Sync + Send>(
        &self,
        addr: Addr<Self::Actor>,
        client: Client,
        stream: S,
        group_name: S,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (mut read, mut write) = client
            .connect_persistent_subscription(stream, group_name, &Default::default())
            .await?;

        while let Some(event) = read.try_next().await? {
            let event = match event {
                SubEvent::EventAppeared(e) => e,
                _ => continue,
            };

            let recorded_event = event.event.as_ref().ok_or("RecordedEvent is None")?;

            self.handle(addr.to_owned(), recorded_event).await?;

            write.ack_event(event).await.ok();
        }

        Ok(())
    }
}
