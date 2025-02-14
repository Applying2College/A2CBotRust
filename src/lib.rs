pub mod mod_tools;
pub mod structs;
pub mod utils;
use std::sync::Arc;

use serenity::{
    all::{GatewayIntents, Permissions},
    Client,
};
use structs::{app_data::AppData, config::Config, handler::Handler};

pub async fn run_bot(config: Config) {
    let intents = GatewayIntents::all();
    let _permission = Permissions::all();
    let app_data = Arc::new(AppData::new());
    let mut client = Client::builder(&config.token, intents)
        .event_handler(Handler::new(config, app_data))
        .await
        .expect("Err creating client");
    client.start().await.expect("can start client")
}
