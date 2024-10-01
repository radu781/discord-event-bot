use clap::Parser;
mod anilist;
mod messages;
use messages::message_type::MessageType;
use serenity::all::standard::macros::command;
use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler {
    cli: Cli,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let text = self.cli.type_.message().await;
        let id = self.cli.type_.channel_id();
        id.send_message(&ctx.http, text)
            .await
            .expect("Could not send message");
        std::process::exit(0);
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    type_: MessageType,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("token").expect("Expected a token in the environment");
    let args = Cli::parse();
    println!("{:?}", args);
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { cli: args })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {why:?}");
    }
}
