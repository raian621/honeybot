use poise::serenity_prelude::{self as serenity, Error};

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

    // Keep bot from responding to itself infinitely:
    if new_message.author.id == ctx.cache.current_user().id {
        return Ok(());
    }

    let response = data
        .datastore
        .get_message_response(new_message.guild_id.unwrap(), new_message.channel_id)
        .await
        .unwrap_or(MessageResponse::Nothing);
    match response {
        MessageResponse::Respond => {
            respond_to_message(ctx, new_message).await?;
        }
        MessageResponse::Nothing => (),
        _ => todo!("unimplemented"),
    };

    Ok(())
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
