use moka::future::Cache;
use poise::serenity_prelude::{self as serenity};
use std::sync::Arc;

use crate::commands::ChannelMessageResponse;

pub struct ContextData {
    pub cache: Arc<ContextDataCache>,
}

pub struct ContextDataCache {
    pub subscribed_channel_responses:
        Cache<(serenity::GuildId, serenity::ChannelId), ChannelMessageResponse>,
}

impl Default for ContextData {
    fn default() -> Self {
        Self {
            cache: Arc::new(Default::default()),
        }
    }
}

impl Default for ContextDataCache {
    fn default() -> Self {
        Self {
            subscribed_channel_responses: Cache::new(10_000),
        }
    }
}
