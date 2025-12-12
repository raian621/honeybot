use poise::serenity_prelude::{self as serenity};

#[derive(Debug, Clone, Copy, poise::ChoiceParameter)]
pub enum MessageResponse {
    #[name = "ban"]
    Ban = 0,
    #[name = "kick"]
    Kick = 1,
    #[name = "respond"]
    Respond = 2,
    #[name = "nothing"]
    Nothing = 3,
}

pub struct MessageResponseConfig {
    pub guild_id: serenity::GuildId,
    pub channel_id: serenity::ChannelId,
    pub response: MessageResponse,
}
