use std::path::Path;

use poise::serenity_prelude as serenity;
use sqlx::{Sqlite, migrate::Migrator, sqlite::SqliteConnectOptions};

use crate::datastore::{
    models::{MessageResponse, MessageResponseConfig},
    traits::{DatastoreReader, DatastoreWriter},
};

pub struct Database {
    pool: sqlx::Pool<Sqlite>,
}

impl DatastoreReader for Database {
    async fn get_message_response(
        &self,
        guild_id: i64,
        channel_id: i64,
    ) -> Option<MessageResponse> {
        let response: Option<i64> = sqlx::query_scalar(
            "SELECT response FROM message_responses WHERE guild_id = ? AND channel_id = ?",
        )
        .bind(guild_id)
        .bind(channel_id)
        .fetch_one(&self.pool)
        .await
        .ok();
        if let Some(response) = response {
            Some(MessageResponse::from(response as usize))
        } else {
            None
        }
    }
}

impl DatastoreWriter for Database {
    async fn delete_message_response_config(&self, guild_id: i64, channel_id: i64) -> Option<()> {
        let result =
            sqlx::query("DELETE FROM message_responses WHERE guild_id = ? AND channel_id = ?")
                .bind(guild_id)
                .bind(channel_id)
                .execute(&self.pool)
                .await;
        if result.is_err() { None } else { Some(()) }
    }

    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Option<()> {
        let result = sqlx::query(
            "INSERT INTO message_responses (guild_id, channel_id, response) VALUES (?, ?, ?)",
        )
        .bind(message_response_config.guild_id)
        .bind(message_response_config.channel_id)
        .bind(message_response_config.response as i64)
        .execute(&self.pool)
        .await;
        if result.is_err() { None } else { Some(()) }
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
            guild_id: 12345678,
            channel_id: 87654321,
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
