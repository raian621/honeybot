mod commands;
mod context_data;
mod datastore;
mod event_handler;

use clap::Parser;
use std::sync::Arc;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity};

use crate::{
    context_data::ContextData,
    datastore::{Datastore, database::Database},
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
    let args = Args::parse();

    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let database = Database::new(&args.db_path.unwrap_or("honeybot.db".to_string())).await;
    database
        .apply_migrations(args.migrations_path.unwrap_or("./migrations".to_string()))
        .await;
    let datastore = Arc::new(Datastore::new(Default::default(), database));
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::listen(), commands::unlisten()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::event_handler(ctx, event, framework, data))
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
