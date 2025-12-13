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
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<MessageResponse> {
        let response: Option<i64> = sqlx::query_scalar(
            "SELECT response FROM message_responses WHERE guild_id = ? AND channel_id = ?",
        )
        .bind(guild_id.get() as i64)
        .bind(channel_id.get() as i64)
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
    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<()> {
        let result =
            sqlx::query("DELETE FROM message_responses WHERE guild_id = ? AND channel_id = ?")
                .bind(guild_id.get() as i64)
                .bind(channel_id.get() as i64)
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
        .bind(message_response_config.guild_id.get() as i64)
        .bind(message_response_config.channel_id.get() as i64)
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
