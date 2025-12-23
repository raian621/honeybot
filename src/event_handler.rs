use std::sync::Arc;

use poise::serenity_prelude::{self as serenity, EventHandler, async_trait};

use crate::datastore::{Datastore, models::MessageResponse, traits::DatastoreReader};

pub struct HoneybotEventHandler {
    datastore: Arc<Datastore>,
}

impl HoneybotEventHandler {
    pub fn new(datastore: Arc<Datastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl EventHandler for HoneybotEventHandler {
    async fn message(&self, ctx: serenity::Context, new_message: serenity::Message) {
        // The bot shouldn't ban, kick, or respond to itself (even if it would be hilarious)
        if new_message.author.id == ctx.cache.current_user().id {
            return;
        }

        let guild_id = new_message.guild_id.unwrap();
        let channel_id = new_message.channel_id;
        let response = self
            .datastore
            .get_message_response(guild_id, channel_id)
            .await
            .unwrap_or(MessageResponse::Nothing);

        // I feel like this is not the best way to get the guild...
        let guild = (*new_message.guild(&ctx.cache).unwrap()).clone();

        match response {
            MessageResponse::Respond => {
                let result = new_message
                    .reply(ctx, "Are you lost? You shouldn't be in this channel...")
                    .await;
                if let Err(why) = result {
                    tracing::error!("Error responding to user: {why:?}");
                }
            }
            MessageResponse::Kick => guild
                .kick_with_reason(ctx, new_message.author.id, "posted in a honeypot channel")
                .await
                .unwrap_or_else(|err| {
                    tracing::error!("Error kicking user: {err:?}");
                }),
            MessageResponse::Ban => guild
                .ban_with_reason(
                    ctx,
                    new_message.author.id,
                    7,
                    "posted in a honeypot channel",
                )
                .await
                .unwrap_or_else(|err| {
                    tracing::error!("Error banning user: {err:?}");
                }),
            MessageResponse::Nothing => (),
        };
    }
}
