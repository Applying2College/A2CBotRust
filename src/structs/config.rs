use std::{collections::HashMap, env, fs::File, io::Read};

use serde::{Deserialize, Serialize};
use serenity::all::GuildId;
use strum::Display;

use crate::{mod_tools::slow_mode::SlowMode, utils::constants::setup_constants};

use super::{clap_args::BotArgs, discord_command::DiscordCommand};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum Commands {
    SlowMode,
    Bonk,
}

impl Commands {
    pub fn get_discord_command(&self) -> Box<dyn DiscordCommand> {
        match self {
            Commands::SlowMode => Box::new(SlowMode),
            Commands::Bonk => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CommandConfig {
    #[serde(default)]
    pub enable_all: bool,
    #[serde(default)]
    pub enabled_users: Vec<i64>,
    #[serde(default)]
    pub enabled_roles: Vec<i64>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub commands: HashMap<Commands, CommandConfig>,
    #[serde(default)]
    pub token: String,
    pub guild_id: GuildId
}

impl Config {
    pub fn new(args: BotArgs) -> Self {
        let file_name = format!("config/{}.yaml", args.config);
        let mut file = File::open(file_name).expect("Config file does not exist");
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)
            .expect("can't read config file");
        let mut config: Self = serde_yml::from_slice(&file_bytes).expect("Is a valid config file");
        if config.token.is_empty() {
            if let Some(arg_token) = args.token {
                config.token = arg_token;
            } else {
                config.token = env::var(setup_constants::DISCORD_TOKEN)
                    .expect("Must give token in config, arg or env");
            }
        }
        config
    }
}
