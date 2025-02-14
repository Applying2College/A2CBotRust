use std::sync::Arc;

use serenity::{
    all::{Context, EventHandler, Message, Ready},
    async_trait,
};

use super::{app_data::AppData, config::Config};

pub struct Handler {
    config: Config,
    app_data: Arc<AppData>,
}

impl Handler {
    pub fn new(config: Config, app_data: Arc<AppData>) -> Self {
        Self { config, app_data }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        for command in self.config.commands.keys() {
            command
                .get_discord_command()
                .init(self.app_data.clone(),&ctx)
                .await
                .expect(&format!("Can initialize {command}"));
        }
        println!("{} is connected!", ready.user.name);
    }
}
