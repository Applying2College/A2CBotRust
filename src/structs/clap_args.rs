use clap::{arg, command, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct BotArgs {
    #[arg(short, long)]
    pub config: String,
    #[arg(short, long)]
    pub token: Option<String>,
}
