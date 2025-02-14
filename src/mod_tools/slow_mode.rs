use std::{
    cmp::min,
    sync::
        Arc
    ,
    time::Duration,
};

use serenity::{all::{CacheHttp, Channel, Context, EditChannel, Message}, async_trait};

use crate::structs::{app_data::AppData, discord_command::DiscordCommand};

pub struct SlowModeChannel {
    pub message_count: usize,
    pub message_rate: usize,
    pub increment: usize,
    pub cap: usize,
}

pub struct SlowMode;
#[async_trait]
impl DiscordCommand for SlowMode {
    async fn process_message(
        &self,
        app_data: &AppData,
        _ctx: &Context,
        msg: &Message,
    ) -> anyhow::Result<()> {
        if let Some(mut chanel_info) = app_data.channel_count.get_mut(&msg.channel_id) {
            chanel_info.message_count += 1;
        }
        Ok(())
    }
    async fn init(&self, app_data: Arc<AppData>, ctx: &Context) -> anyhow::Result<()> {
        let ctx_clone = ctx.clone();
        tokio::task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                for mut channel in app_data.channel_count.iter_mut() {
                    let slow_amount = min(
                        channel.increment * channel.message_count / channel.message_rate,
                        channel.cap,
                    );
                    let builder = EditChannel::new()
                        .rate_limit_per_user(slow_amount.try_into().unwrap_or(u16::MAX));
                    if let Ok(Channel::Guild(mut channel)) =
                        channel.key().to_channel(ctx_clone.http()).await
                    {
                        let _ = channel.edit(ctx_clone.http(), builder).await;
                    }
                    channel.message_count = channel.message_count.saturating_sub(1);
                }
            }
        });
        Ok(())
    }
}
