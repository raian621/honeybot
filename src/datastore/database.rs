use std::path::Path;

use poise::serenity_prelude as serenity;
use sqlx::{Sqlite, migrate::Migrator, sqlite::SqliteConnectOptions};

use crate::datastore::{
    errors::Error,
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

pub struct Database {
    pool: sqlx::Pool<Sqlite>,
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
        response
            .map(MessageResponse::from)
            .or(Err(Error::DatabaseEntryNotFound))
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
        let result = sqlx::query(
            "INSERT INTO message_responses (guild_id, channel_id, response) VALUES (?, ?, ?)",
        )
        .bind(message_response_config.guild_id.get() as i64)
        .bind(message_response_config.channel_id.get() as i64)
        .bind(message_response_config.response as i64)
        .execute(&self.pool)
        .await;
        if result.is_ok() {
            return Ok(());
        }
        let result = sqlx::query(
            "UPDATE message_responses SET response = ? WHERE guild_id = ? AND channel_id = ?",
        )
        .bind(message_response_config.response as i64)
        .bind(message_response_config.guild_id.get() as i64)
        .bind(message_response_config.channel_id.get() as i64)
        .execute(&self.pool)
        .await;
        if result.is_err() {
            return Err(Error::DatabaseUnexpectedErr);
        } else {
            Ok(())
        }
    }
}

impl Database {
    pub async fn apply_migrations(&self, migrations_path: String) {
        Migrator::new(Path::new(&migrations_path))
            .await
            .unwrap()
            .run(&self.pool)
            .await
            .unwrap();
    }

    pub async fn new(filename: &str) -> Self {
        Self {
            pool: sqlx::Pool::connect_with(
                SqliteConnectOptions::new()
                    .filename(filename)
                    .create_if_missing(true),
            )
            .await
            .unwrap(),
        }
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

        delete_test_db(db).await;
    }
}
