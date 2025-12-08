use poise::{
    Context,
    serenity_prelude::{self as serenity, Error},
};

use crate::context_data;

#[derive(Debug, poise::ChoiceParameter)]
pub enum ChannelMessageResponse {
    #[name = "ban"]
    Ban,
    #[name = "kick"]
    Kick,
    #[name = "respond"]
    Respond,
}

#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
pub async fn listen(
    ctx: Context<'_, context_data::ContextData, Error>,
    #[description = "Channel to listen to"] channel: serenity::Channel,
    #[description = "Action for each new message in channel"] response: ChannelMessageResponse,
) -> Result<(), Error> {
    let channel_id = channel.id();
    let guild_channel = channel.guild().unwrap();
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
    let guild_channel = channel.guild().unwrap();
    guild_channel.say(ctx, "Listening").await?;
    ctx.send(
        poise::CreateReply::default()
            .content(format!("Unlistening to channel <#{}>", channel_id))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}
