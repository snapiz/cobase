use actix::{Handler, ResponseFuture};
use eventstore::EventData;
use serde::Deserialize;
use validator::Validate;

use crate::bus::{Bus, Message};
use crate::error::Error;
use crate::event::Domain;

use super::event::{Created, Event};

#[derive(Deserialize, Validate, Default)]
pub struct CreateGroupCommand {
    #[validate(length(min = 3))]
    pub name: String,
}

impl Handler<Message<CreateGroupCommand>> for Bus {
    type Result = ResponseFuture<Result<(), Error>>;

    fn handle(
        &mut self,
        msg: Message<CreateGroupCommand>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let client = self.eventstore_client.clone();

        Box::pin(async move {
            msg.1.validate()?;

            let event = EventData::json(
                Event::Created,
                Created {
                    user_id: msg.0,
                    name: msg.1.name,
                },
            )?;

            client
                .append_to_stream(Domain::Group.to_stream_id(), &Default::default(), event)
                .await??;

            Ok(())
        })
    }
}
