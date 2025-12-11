use poise::serenity_prelude::{self as serenity, Error};

use crate::{commands::ChannelMessageResponse, context_data};

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

    // Keep bot from responding to itself infinitely:
    if new_message.author.id == ctx.cache.current_user().id {
        return Ok(());
    }

    let response =
        get_response_for_message(data, new_message.guild_id.unwrap(), new_message.channel_id)
            .await
            .unwrap_or(ChannelMessageResponse::Nothing);
    match response {
        ChannelMessageResponse::Respond => {
            respond_to_message(ctx, new_message).await?;
        }
        ChannelMessageResponse::Nothing => (),
        _ => todo!("unimplemented"),
    };

    Ok(())
}

async fn get_response_for_message(
    data: &context_data::ContextData,
    guild_id: serenity::GuildId,
    channel_id: serenity::ChannelId,
) -> Option<ChannelMessageResponse> {
    data.cache
        .subscribed_channel_responses
        .get(&(guild_id, channel_id))
        .await
}

async fn respond_to_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
) -> Result<(), Error> {
    message
        .reply(ctx, "Are you lost? You shouldn't be in this channel...")
        .await?;
    Ok(())
}
