mod commands;
mod context_data;
mod datastore;
mod event_handler;

use clap::Parser;
use std::sync::Arc;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, Error};

use crate::{
    context_data::ContextData,
    datastore::{Datastore, DatastoreOptions, database::DatabaseOptions},
    event_handler::HoneybotEventHandler,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to sqlite db file
    #[arg(short, long)]
    db_path: Option<String>,

    /// Path to migrations directory
    #[arg(short, long)]
    migrations_path: Option<String>,
}

#[tokio::main]
async fn main() {
    // Start tracing logger:
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let args = Args::parse();

    let datastore = Arc::new(
        Datastore::new_with_options(&DatastoreOptions {
            database_options: DatabaseOptions {
                filename: args.db_path.unwrap_or("honeybot.db".to_string()),
                migrations_path: args.migrations_path.unwrap_or("./migrations".to_string()),
            },
            cache_options: Default::default(),
        })
        .await,
    );

    // Poise boilerplate to configure bot:
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::listen(), commands::unlisten()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
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
                Ok(ContextData::new(datastore.clone()))
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, ContextData, Error>,
    data: &ContextData,
) -> Result<(), Error> {
    event
        .clone()
        .dispatch(
            ctx.clone(),
            &HoneybotEventHandler::new(data.datastore.clone()),
        )
        .await;
    Ok(())
}
