use poise::{
    Context,
    serenity_prelude::{self as serenity, Error},
};
use tracing::{Level, event};

use crate::{
    context_data,
    datastore::{
        models::{MessageResponse, MessageResponseConfig},
        prelude::*,
    },
};

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn listen(
    ctx: Context<'_, context_data::ContextData, Error>,
    #[description = "Channel to listen to"] channel: serenity::Channel,
    #[description = "Action for each new message in channel"] response: MessageResponse,
) -> Result<(), Error> {
    let channel_id = channel.id();
    let guild_channel = channel.guild().unwrap();
    match ctx
        .data()
        .datastore
        .insert_message_response_config(&MessageResponseConfig {
            guild_id: ctx.guild_id().unwrap(),
            channel_id,
            response,
        })
        .await
    {
        Ok(_) => {
            guild_channel
                .say(
                    ctx,
                    format!(
                        "**Do not** post in this channel unless you want to be {}",
                        match response {
                            MessageResponse::Ban => "banned",
                            MessageResponse::Kick => "kicked",
                            MessageResponse::Respond => "mocked",
                            MessageResponse::Nothing => "ignored",
                        },
                    ),
                )
                .await?;
            ctx.send(
                poise::CreateReply::default()
                    .content(format!(
                        "Listening to channel <#{channel_id}>, prepared to take action `{response:?}`"
                    ))
                    .ephemeral(true),
            )
            .await?;
        }
        Err(why) => {
            event!(Level::WARN, "Error listening to channel: {why:?}");
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Error listening to channel >#{channel_id}>"))
                    .ephemeral(true),
            )
            .await?;
        }
    }
    Ok(())
}

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn unlisten(
    ctx: Context<'_, context_data::ContextData, Error>,
    #[description = "Channel to unlisten to"] channel: serenity::Channel,
) -> Result<(), Error> {
    let channel_id = channel.id();
    let result = ctx
        .data()
        .datastore
        .delete_message_response_config(ctx.guild_id().unwrap(), channel_id)
        .await;
    match result {
        Ok(_) => {
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Unlistening to channel <#{channel_id}>"))
                    .ephemeral(true),
            )
            .await?;
        }
        Err(why) => {
            event!(Level::WARN, "Error unlistening to channel: {why:?}");
            ctx.send(
                poise::CreateReply::default()
                    .content(format!("Error unlistening to channel >#{channel_id}>"))
                    .ephemeral(true),
            )
            .await?;
        }
    }
    Ok(())
}
