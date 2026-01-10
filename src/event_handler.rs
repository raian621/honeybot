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
        let user_id = new_message.author.id;

        match response {
            MessageResponse::Respond => {
                let result = new_message
                    .reply(&ctx, "Are you lost? You shouldn't be in this channel...")
                    .await;
                if let Err(why) = result {
                    tracing::error!("Error responding to user: {why:?}");
                }
            }
            MessageResponse::Kick => guild
                .kick_with_reason(&ctx, user_id, "posted in a honeypot channel")
                .await
                .unwrap_or_else(|err| {
                    tracing::error!("Error kicking user: {err:?}");
                }),
            MessageResponse::Ban => guild
                .ban_with_reason(&ctx, user_id, 7, "posted in a honeypot channel")
                .await
                .unwrap_or_else(|err| {
                    tracing::error!("Error banning user: {err:?}");
                }),
            MessageResponse::Nothing => (),
        };

        if let Ok(logging_channel_id) = self.datastore.get_logging_channel(guild_id).await {
            let logging_channel = guild.channels.get(&logging_channel_id);
            if logging_channel.is_none() {
                tracing::error!("Logging channel `{logging_channel_id}` not found!");
                return;
            }
            log_action_in_channel(&ctx, response, user_id, logging_channel.unwrap()).await;
        } else {
            tracing::warn!("Logging channel not found for guild `{guild_id}`");
        }
    }
}

async fn log_action_in_channel(
    ctx: &serenity::Context,
    action: MessageResponse,
    user_id: serenity::UserId,
    logging_channel: &serenity::GuildChannel,
) {
    let action_str = match action {
        MessageResponse::Ban => "Banned",
        MessageResponse::Kick => "Kicked",
        MessageResponse::Nothing => "Nothing done to",
        MessageResponse::Respond => "Warned",
    };
    let result = logging_channel
        .say(
            ctx,
            format!("{action_str} user <@{user_id}> for posting in the honeypot channel.",),
        )
        .await;
    match result {
        Ok(_) => (),
        Err(why) => {
            tracing::warn!(
                "Error logging action in channel `{}`: {why:?}",
                logging_channel.id
            );
        }
    }
}
