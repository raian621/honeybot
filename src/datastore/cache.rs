use moka::future::Cache;
use poise::serenity_prelude::{self as serenity};

use crate::datastore::{
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

pub struct DatabaseCache {
    subscribed_channel_responses: Cache<(i64, i64), MessageResponse>,
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
        guild_id: i64,
        channel_id: i64,
    ) -> Option<MessageResponse> {
        self.subscribed_channel_responses
            .get(&(guild_id, channel_id))
            .await
    }
}

impl DatastoreWriter for DatabaseCache {
    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Option<()> {
        Some(
            self.subscribed_channel_responses
                .insert(
                    (
                        message_response_config.guild_id,
                        message_response_config.channel_id,
                    ),
                    message_response_config.response,
                )
                .await,
        )
    }

    async fn delete_message_response_config(&self, guild_id: i64, channel_id: i64) -> Option<()> {
        if self
            .subscribed_channel_responses
            .remove(&(guild_id, channel_id))
            .await
            .is_none()
        {
            None
        } else {
            Some(())
        }
    }
}
