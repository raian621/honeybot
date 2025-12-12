use std::path::Path;

use poise::serenity_prelude as serenity;
use sqlx::{Sqlite, migrate::Migrator, sqlite::SqliteConnectOptions};

use crate::datastore::{models::MessageResponse, traits::DatastoreReader};

pub struct Database {
    pool: sqlx::Pool<Sqlite>,
}

impl DatastoreReader for Database {
    async fn get_message_response(
        &self,
        _guild_id: serenity::GuildId,
        _channel_id: serenity::ChannelId,
    ) -> Option<MessageResponse> {
        todo!();
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
