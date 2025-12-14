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

#[cfg(test)]
mod test_utils;

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
        let response = self.cache.get_message_response(guild_id, channel_id).await;
        if response.is_some() {
            return response;
        }
        let response = self
            .database
            .get_message_response(guild_id, channel_id)
            .await;
        if response.is_none() {
            return response;
        }
        match self
            .database
            .insert_message_response_config(&MessageResponseConfig {
                guild_id,
                channel_id,
                response: response.unwrap(),
            })
            .await
        {
            Some(_) => response,
            None => None,
        }
    }
}

impl DatastoreWriter for Datastore {
    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Option<()> {
        match self
            .database
            .insert_message_response_config(message_response_config)
            .await
        {
            None => None,
            Some(_) => {
                // Only write to cache if db write was successful
                self.cache
                    .insert_message_response_config(message_response_config)
                    .await
            }
        }
    }

    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<()> {
        match self
            .database
            .delete_message_response_config(guild_id, channel_id)
            .await
        {
            None => None,
            Some(_) => {
                self.cache
                    .delete_message_response_config(guild_id, channel_id)
                    .await
            }
        }
    }
}

impl Datastore {
    pub fn new(cache: DatabaseCache, database: Database) -> Self {
        Self { cache, database }
    }
}

#[cfg(test)]
mod tests {
    use crate::datastore::test_utils::{delete_test_db, get_test_db};

    use super::*;

    #[tokio::test]
    async fn create_read_and_delete_message_response_config() {
        let db = get_test_db().await;

        // Create the message response configuration in db
        let message_response = MessageResponseConfig {
            guild_id: serenity::GuildId::from(12345678),
            channel_id: serenity::ChannelId::from(87654321),
            response: MessageResponse::Ban,
        };
        let opt = db.insert_message_response_config(&message_response).await;
        assert_eq!(opt, Some(()));

        // Read the message response for guild and channel id
        let opt = db
            .get_message_response(message_response.guild_id, message_response.channel_id)
            .await;
        assert_eq!(opt, Some(message_response.response));

        // Delete the message response config
        let opt = db
            .delete_message_response_config(message_response.guild_id, message_response.channel_id)
            .await;
        assert_eq!(opt, Some(()));

        // Message response config should be deleted

        let opt = db
            .get_message_response(message_response.guild_id, message_response.channel_id)
            .await;
        assert_eq!(opt, None);

        delete_test_db(db).await;
    }
}
