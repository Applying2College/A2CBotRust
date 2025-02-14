use std::sync::Arc;

use serenity::{
    all::{Context, Message},
    async_trait,
};

use super::app_data::AppData;

#[async_trait]
pub trait DiscordCommand: Send + Sync {
    async fn process_message(
        &self,
        _app_data: &AppData,
        _ctx: &Context,
        _msg: &Message,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn process_command(
        &self,
        _app_data: &AppData,
        _ctx: &Context,
        _msg: &Message,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn init(&self, _app_data: Arc<AppData>, _ctx: &Context) -> anyhow::Result<()> {
        Ok(())
    }
}
