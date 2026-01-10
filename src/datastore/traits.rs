use poise::serenity_prelude::{self as serenity};

use crate::datastore::{
    errors::Error,
    models::{MessageResponse, MessageResponseConfig},
};

pub trait DatastoreReader {
    async fn get_message_response(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<MessageResponse, Error>;

    async fn get_logging_channel(
        &self,
        guild_id: serenity::GuildId,
    ) -> Result<serenity::ChannelId, Error>;
}

pub trait DatastoreWriter {
    async fn insert_message_response_config(
        &self,
        message_response_config: &MessageResponseConfig,
    ) -> Result<(), Error>;

    async fn delete_message_response_config(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<(), Error>;

    async fn insert_logging_channel(
        &self,
        guild_id: serenity::GuildId,
        channel_id: serenity::ChannelId,
    ) -> Result<(), Error>;

    // Might add a command to use this method later. As of this commit, this method is only used in
    // tests.
    #[allow(dead_code)]
    async fn delete_logging_channel(&self, guild_id: serenity::GuildId) -> Result<(), Error>;
}
