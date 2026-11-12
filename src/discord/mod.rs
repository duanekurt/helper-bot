pub mod handler;

use ::serenity::{all::GatewayIntents, Client};
use anyhow::Result;

pub async fn start_bot(token: String) -> Result<()> {

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(handler::Handler)
        .await?;

    client.start().await?;

    Ok(())
}
