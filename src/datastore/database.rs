use std::path::Path;

use poise::serenity_prelude::{self as serenity};
use sqlx::{Sqlite, migrate::Migrator, sqlite::SqliteConnectOptions};

use crate::datastore::{
    errors::Error,
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

pub struct Database {
    pool: sqlx::Pool<Sqlite>,
}

pub struct DatabaseOptions {
    pub filename: String,
    pub migrations_path: String,
}

impl DatastoreReader for Database {
    async fn get_message_response(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<MessageResponse, Error> {
        let response: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT response FROM message_responses WHERE guild_id = ? AND channel_id = ?",
        )
        .bind(guild_id.get() as i64)
        .bind(channel_id.get() as i64)
        .fetch_one(&self.pool)
        .await;
        match response {
            Err(sqlx::Error::RowNotFound) => Err(Error::DatabaseEntryNotFound),
            Err(why) => Err(Error::DatabaseUnexpectedErr(format!("{why:?}"))),
            Ok(response) => Ok(MessageResponse::from(response)),
        }
    }

    async fn get_logging_channel(
        &self,
        guild_id: serenity::GuildId,
    ) -> Result<serenity::ChannelId, Error> {
        let response: Result<i64, sqlx::Error> =
            sqlx::query_scalar("SELECT channel_id FROM logging_channels WHERE guild_id = ?")
                .bind(guild_id.get() as i64)
                .fetch_one(&self.pool)
                .await;
        match response {
            Err(sqlx::Error::RowNotFound) => Err(Error::DatabaseEntryNotFound),
            Err(why) => Err(Error::DatabaseUnexpectedErr(format!("{why:?}"))),
            Ok(response) => Ok(serenity::ChannelId::new(response as u64)),
        }
    }
}

impl DatastoreWriter for Database {
    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<(), Error> {
        let result =
            sqlx::query("DELETE FROM message_responses WHERE guild_id = ? AND channel_id = ?")
                .bind(guild_id.get() as i64)
                .bind(channel_id.get() as i64)
                .execute(&self.pool)
                .await;
        result.map(|_| ()).or(Err(Error::DatabaseEntryNotFound))
    }

    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Result<(), Error> {
        // Try inserting into the db
        let result = sqlx::query(concat!(
            "INSERT INTO message_responses (guild_id, channel_id, response) VALUES ($1, $2, $3) ",
            "ON CONFLICT(guild_id, channel_id) DO UPDATE SET response = $3"
        ))
        .bind(message_response_config.guild_id.get() as i64)
        .bind(message_response_config.channel_id.get() as i64)
        .bind(message_response_config.response as i64)
        .execute(&self.pool)
        .await;
        match result {
            Err(why) => Err(Error::DatabaseUnexpectedErr(format!("{why:?}"))),
            Ok(_) => Ok(()),
        }
    }

    async fn delete_logging_channel(&self, guild_id: serenity::GuildId) -> Result<(), Error> {
        let result = sqlx::query("DELETE FROM logging_channels WHERE guild_id = ?")
            .bind(guild_id.get() as i64)
            .execute(&self.pool)
            .await;
        result.map(|_| ()).or(Err(Error::DatabaseEntryNotFound))
    }

    async fn insert_logging_channel(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<(), Error> {
        let result = sqlx::query(concat!(
            "INSERT INTO logging_channels (guild_id, channel_id) VALUES ($1, $2) ",
            "ON CONFLICT(guild_id) DO UPDATE SET channel_id = $2"
        ))
        .bind(guild_id.get() as i64)
        .bind(channel_id.get() as i64)
        .execute(&self.pool)
        .await;
        match result {
            Err(why) => Err(Error::DatabaseUnexpectedErr(format!("{why:?}"))),
            Ok(_) => Ok(()),
        }
    }
}

impl Database {
    pub async fn new(options: &DatabaseOptions) -> Self {
        let db = Self {
            pool: sqlx::Pool::connect_with(
                SqliteConnectOptions::new()
                    .filename(&options.filename)
                    .create_if_missing(true),
            )
            .await
            .unwrap(),
        };
        Migrator::new(Path::new(&options.migrations_path))
            .await
            .unwrap()
            .run(&db.pool)
            .await
            .unwrap();
        db
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
        let mut message_response = MessageResponseConfig {
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

        // Update the message response for guild and channel id
        message_response.response = MessageResponse::Kick;
        let result = db.insert_message_response_config(&message_response).await;
        assert_eq!(result, Ok(()));

        // Read the updated message response for guild and channel id
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

    #[tokio::test]
    #[serial]
    async fn create_read_and_delete_logging_channel() {
        let db = get_test_db().await;

        let guild_id = serenity::GuildId::new(12345678);
        let channel_id = serenity::ChannelId::new(87654321);
        let result = db.insert_logging_channel(guild_id, channel_id).await;
        assert_eq!(result, Ok(()));

        let result = db.get_logging_channel(guild_id).await;
        assert_eq!(result, Ok(channel_id));

        let channel_id = serenity::ChannelId::new(01234567);
        let result = db.insert_logging_channel(guild_id, channel_id).await;
        assert_eq!(result, Ok(()));

        let result = db.get_logging_channel(guild_id).await;
        assert_eq!(result, Ok(channel_id));

        let result = db.delete_logging_channel(guild_id).await;
        assert_eq!(result, Ok(()));

        let result = db.get_logging_channel(guild_id).await;
        assert_eq!(result, Err(Error::DatabaseEntryNotFound));
    }
}
