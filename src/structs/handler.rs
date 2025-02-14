use std::sync::Arc;

use serenity::{
    all::{CacheHttp, Context, EventHandler, Interaction, Message, Ready},
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

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command_interaction) = interaction {

            for command in self.config.commands.keys() {
                if let Err(e) = command
                    .get_discord_command()
                    .process_command(&self.app_data,&ctx,&command_interaction)
                    .await{
                        log::error!("Error handling {command} message: {e}")
                    }
            }



        }
    }
    async fn message(&self, ctx: Context, msg: Message) {
        for command in self.config.commands.keys() {
            if let Err(e) = command
                .get_discord_command()
                .process_message(&self.app_data,&ctx,&msg)
                .await{
                    log::error!("Error handling {command} message: {e}")
                }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let mut commands = Vec::new();
        for command in self.config.commands.keys() {
            command
                .get_discord_command()
                .init(self.app_data.clone(),&ctx)
                .await
                .expect(&format!("Can't initialize {command}"));
        }
        for command in self.config.commands.keys() {
            commands.append(&mut command
                .get_discord_command()
                .get_commands(&self.app_data,&self.config,&ctx)
                .await
                .expect(&format!("Can get all {command}")));
        }
        self.config.guild_id.set_commands(ctx.http(), commands).await.expect("can set commands");
        println!("{} is connected!", ready.user.name);
    }
}
