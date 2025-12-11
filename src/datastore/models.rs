use poise::serenity_prelude::{self as serenity};

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum MessageResponse {
    #[name = "ban"]
    Ban,
    #[name = "kick"]
    Kick,
    #[name = "respond"]
    Respond,
    #[name = "nothing"]
    Nothing,
}

pub struct MessageResponseConfig {
    pub guild_id: serenity::GuildId,
    pub channel_id: serenity::ChannelId,
    pub response: MessageResponse,
}
