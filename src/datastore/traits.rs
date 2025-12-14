use poise::serenity_prelude::{self as serenity};

use crate::datastore::models::{MessageResponse, MessageResponseConfig};

pub trait DatastoreReader {
    async fn get_message_response(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<MessageResponse>;
}

pub trait DatastoreWriter {
    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Option<()>;

    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Option<()>;
}
