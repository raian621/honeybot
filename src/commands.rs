use poise::{
    Context,
    serenity_prelude::{self as serenity, Error},
};

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
    ctx.data()
        .datastore
        .insert_message_response_config(&MessageResponseConfig {
            guild_id: ctx.guild_id().unwrap(),
            channel_id,
            response,
        })
        .await;
    guild_channel.say(ctx, "Listening").await?;
    ctx.send(
        poise::CreateReply::default()
            .content(format!(
                "Listening to channel <#{}>, prepared to take action `{:?}`",
                channel_id, response
            ))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn unlisten(
    ctx: Context<'_, context_data::ContextData, Error>,
    #[description = "Channel to unlisten to"] channel: serenity::Channel,
) -> Result<(), Error> {
    let channel_id = channel.id();
    ctx.data()
        .datastore
        .delete_message_response_config(ctx.guild_id().unwrap(), channel_id)
        .await;
    ctx.send(
        poise::CreateReply::default()
            .content(format!("Unlistening to channel <#{}>", channel_id))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
