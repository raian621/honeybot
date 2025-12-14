use poise::serenity_prelude::{self as serenity};

const BAN: isize = 0;
const KICK: isize = 1;
const RESPOND: isize = 2;
const NOTHING: isize = 3;

#[derive(Debug, Clone, Copy, PartialEq, poise::ChoiceParameter)]
pub enum MessageResponse {
    #[name = "ban"]
    Ban = BAN,
    #[name = "kick"]
    Kick = KICK,
    #[name = "respond"]
    Respond = RESPOND,
    #[name = "nothing"]
    Nothing = NOTHING,
}

impl From<i64> for MessageResponse {
    fn from(value: i64) -> Self {
        match value as isize {
            BAN => MessageResponse::Ban,
            KICK => MessageResponse::Kick,
            RESPOND => MessageResponse::Respond,
            NOTHING => MessageResponse::Nothing,
            _ => panic!("invalid message response"),
        }
    }
}

pub struct MessageResponseConfig {
    pub guild_id: serenity::GuildId,
    pub channel_id: serenity::ChannelId,
    pub response: MessageResponse,
}
