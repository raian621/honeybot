use poise::serenity_prelude::{self as serenity};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageResponse {
    Ban = 0,
    Kick = 1,
    Respond = 2,
    Nothing = 3,
}

impl From<usize> for MessageResponse {
    fn from(value: usize) -> Self {
        const BAN: usize = MessageResponse::Ban as usize;
        const KICK: usize = MessageResponse::Kick as usize;
        const RESPONSE: usize = MessageResponse::Respond as usize;
        const NOTHING: usize = MessageResponse::Nothing as usize;

        match value {
            BAN => MessageResponse::Ban,
            KICK => MessageResponse::Kick,
            RESPONSE => MessageResponse::Respond,
            NOTHING => MessageResponse::Nothing,
            _ => panic!("invalid integer enum conversion"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, poise::ChoiceParameter)]
pub enum DiscordMessageResponse {
    #[name = "ban"]
    Ban,
    #[name = "kick"]
    Kick,
    #[name = "respond"]
    Respond,
    #[name = "nothing"]
    Nothing,
}

impl Into<MessageResponse> for DiscordMessageResponse {
    fn into(self) -> MessageResponse {
        match self {
            DiscordMessageResponse::Ban => MessageResponse::Ban,
            DiscordMessageResponse::Kick => MessageResponse::Kick,
            DiscordMessageResponse::Respond => MessageResponse::Respond,
            DiscordMessageResponse::Nothing => MessageResponse::Nothing,
        }
    }
}

pub struct MessageResponseConfig {
    pub guild_id: i64,
    pub channel_id: i64,
    pub response: MessageResponse,
}
