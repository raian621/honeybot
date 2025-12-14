use moka::future::Cache;
use poise::serenity_prelude::{self as serenity};

use crate::datastore::{
    errors::Error,
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

pub struct DatabaseCache {
    subscribed_channel_responses: Cache<(serenity::GuildId, serenity::ChannelId), MessageResponse>,
}

impl Default for DatabaseCache {
    fn default() -> Self {
        Self {
            subscribed_channel_responses: Cache::new(10_000),
        }
    }
}

impl DatastoreReader for DatabaseCache {
    async fn get_message_response(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<MessageResponse, Error> {
        match self
            .subscribed_channel_responses
            .get(&(guild_id, channel_id))
            .await
        {
            Some(response) => Ok(response),
            None => Err(Error::CacheEntryNotFound),
        }
    }
}

impl DatastoreWriter for DatabaseCache {
    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Result<(), Error> {
        self.subscribed_channel_responses
            .insert(
                (
                    message_response_config.guild_id,
                    message_response_config.channel_id,
                ),
                message_response_config.response,
            )
            .await;
        Ok(())
    }

    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<(), Error> {
        self.subscribed_channel_responses
            .remove(&(guild_id, channel_id))
            .await;
        Ok(())
    }
}
