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

pub struct MessageResponseConfig {
    pub guild_id: serenity::GuildId,
    pub channel_id: serenity::ChannelId,
    pub response: MessageResponse,
}
