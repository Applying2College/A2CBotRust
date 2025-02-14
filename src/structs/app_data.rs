use std::sync::Arc;

use dashmap::DashMap;
use serenity::all::ChannelId;

use crate::mod_tools::slow_mode::SlowModeChannel;

pub struct AppData {
    //We use DashMap here for concurrency support.
    pub channel_count: Arc<DashMap<ChannelId, SlowModeChannel>>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            channel_count: DashMap::new().into(),
        }
    }
}
