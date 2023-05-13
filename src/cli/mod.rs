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

    /// All Spiffo commands
    #[command(subcommand)]
    command: Command
}



