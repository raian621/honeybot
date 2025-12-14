use poise::serenity_prelude::{self as serenity, Error, Message};

use crate::{
    context_data,
    datastore::{models::MessageResponse, traits::DatastoreReader},
};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, context_data::ContextData, Error>,
    data: &context_data::ContextData,
) -> Result<(), Error> {
    tokio::try_join!(listen_for_messages(ctx, event, data))?;
    Ok(())
}

async fn listen_for_messages(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    data: &context_data::ContextData,
) -> Result<(), Error> {
    let new_message = if let serenity::FullEvent::Message { new_message } = event {
        new_message
    } else {
        return Ok(());
    };

    // The bot shouldn't ban, kick, or respond to itself (even if it would be hilarious)
    if new_message.author.id == ctx.cache.current_user().id {
        return Ok(());
    }

    let guild_id = new_message.guild_id.unwrap();
    let channel_id = new_message.channel_id;
    let response = data
        .datastore
        .get_message_response(guild_id, channel_id)
        .await
        .unwrap_or(MessageResponse::Nothing);

    println!("{response:?} {channel_id} {guild_id}");

    // I feel like this is not the best way to get the guild...
    let guild = (*new_message.guild(&ctx.cache).unwrap()).clone();

    match response {
        MessageResponse::Respond => {
            new_message
                .reply(ctx, "Are you lost? You shouldn't be in this channel...")
                .await?;
        }
        MessageResponse::Kick => {
            guild
                .kick_with_reason(ctx, new_message.author.id, "")
                .await?
        }
        MessageResponse::Ban => {
            guild
                .ban_with_reason(ctx, new_message.author.id, 7, "")
                .await?
        }
        MessageResponse::Nothing => (),
    };

    Ok(())
}
