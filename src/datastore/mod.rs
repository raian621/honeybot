use poise::serenity_prelude::{self as serenity};

use crate::datastore::{
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

mod cache;
pub mod models;
pub mod traits;

pub mod prelude {
    pub use super::traits::*;
}

pub struct Datastore {
    cache: cache::DatabaseCache,
}

impl DatastoreReader for Datastore {
    async fn get_message_response(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<MessageResponse> {
        self.cache.get_message_response(guild_id, channel_id).await
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

impl Default for Datastore {
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}
