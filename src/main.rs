mod commands;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::listen(), commands::unlisten()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                match std::env::var("GUILD_ID") {
                    Ok(guild_id) => {
                        let guild_id = guild_id.parse().unwrap();
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            serenity::GuildId::new(guild_id),
                        )
                        .await?
                    }
                    Err(_) => {
                        poise::builtins::register_globally(ctx, &framework.options().commands)
                            .await?
                    }
                }
                Ok(())
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
