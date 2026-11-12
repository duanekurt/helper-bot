use serenity::{
    all::{Context, EventHandler, Message},
    async_trait,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.content == "!ping" {
            if let Err(error) = msg
                .channel_id
                .say(&ctx.http, "Pong!")
                .await
            {
                eprintln!("Failed to send message: {error}");
            }
        }
    }
}