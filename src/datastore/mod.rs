use poise::serenity_prelude::{self as serenity};

use crate::datastore::{
    cache::DatabaseCache,
    database::Database,
    errors::Error,
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

pub mod cache;
pub mod database;
pub mod errors;
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
    ) -> Result<MessageResponse, Error> {
        // Return cached value if it's found
        let result = self.cache.get_message_response(guild_id, channel_id).await;
        if result.is_ok() {
            return result;
        }

        // Read from database after cache miss
        let response = self
            .database
            .get_message_response(guild_id, channel_id)
            .await?;

        // Ignore cache insertion errors
        let _ = self
            .cache
            .insert_message_response_config(&MessageResponseConfig {
                guild_id,
                channel_id,
                response,
            })
            .await;

        Ok(response)
    }
}

impl DatastoreWriter for Datastore {
    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Result<(), Error> {
        self.database
            .insert_message_response_config(message_response_config)
            .await?;
        self.cache
            .insert_message_response_config(message_response_config)
            .await
    }

    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<(), Error> {
        self.database
            .delete_message_response_config(guild_id, channel_id)
            .await?;
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

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::datastore::test_utils::get_test_db;

    use super::*;

    #[tokio::test]
    #[serial]
    async fn create_read_and_delete_message_response_config() {
        let db = get_test_db().await;
        let guild_id = 12345678;
        let channel_id = 87654321;

        // Create the message response configuration in db
        let message_response = MessageResponseConfig {
            guild_id: serenity::GuildId::from(guild_id),
            channel_id: serenity::ChannelId::from(channel_id),
            response: MessageResponse::Ban,
        };
        let result = db.insert_message_response_config(&message_response).await;
        assert_eq!(result, Ok(()));

        // Read the message response for guild and channel id
        let result = db
            .get_message_response(message_response.guild_id, message_response.channel_id)
            .await;
        assert_eq!(result, Ok(message_response.response));

        // Delete the message response config
        let result = db
            .delete_message_response_config(message_response.guild_id, message_response.channel_id)
            .await;
        assert_eq!(result, Ok(()));

        // Message response config should be deleted
        let result = db
            .get_message_response(message_response.guild_id, message_response.channel_id)
            .await;
        assert_eq!(result, Err(Error::DatabaseEntryNotFound));

        // Clean up rows:
        db.delete_message_response_config(
            serenity::GuildId::from(guild_id),
            serenity::ChannelId::from(channel_id),
        )
        .await
        .unwrap();
    }
}
