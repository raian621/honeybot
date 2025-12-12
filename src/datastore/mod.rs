use poise::serenity_prelude::{self as serenity};

use crate::datastore::{
    cache::DatabaseCache,
    database::Database,
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

pub mod cache;
pub mod database;
pub mod models;
pub mod traits;

pub mod prelude {
    pub use super::traits::*;
}

pub struct Datastore {
    cache: cache::DatabaseCache,
    database: database::Database,
}

impl DatastoreReader for Datastore {
    async fn get_message_response(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<MessageResponse> {
        match self.cache.get_message_response(guild_id, channel_id).await {
            Some(response) => Some(response),
            None => {
                let response = self
                    .database
                    .get_message_response(guild_id, channel_id)
                    .await;
                if let Some(response) = response {
                    self.cache
                        .insert_message_response_config(&MessageResponseConfig {
                            guild_id,
                            channel_id,
                            response,
                        })
                        .await;
                }
                response
            }
        }
    }
}

impl DatastoreWriter for Datastore {
    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Option<()> {
        self.cache
            .insert_message_response_config(message_response_config)
            .await
    }

    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<()> {
        self.cache
            .delete_message_response_config(guild_id, channel_id)
            .await
    }
}

impl Datastore {
    pub fn new(cache: DatabaseCache, database: Database) -> Self {
        Self { cache, database }
    }
}
