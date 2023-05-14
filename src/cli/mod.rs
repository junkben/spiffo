mod command;
mod settings;

use clap::Parser;

pub use {command::*, settings::*};

#[derive(Parser, Getters)]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Enable debug mode
    #[arg(long, env = "DEBUG", default_value_t = true)]
    debug: bool,

    /// Filepath where the Zomboid Server folder lives
    #[arg(long, env = "SERVER_FILEPATH", default_value_t = format!("~/Zomboid/Server"))]
    server_filepath: String,

    /// Filepath where the Zomboid Server folder lives
    #[arg(long, env = "SERVER_CONFIG_NAME", default_value_t = format!("servertest.ini"))]
    server_config_name: String,

    /// All Spiffo commands
    #[command(subcommand)]
    command: Command
}



