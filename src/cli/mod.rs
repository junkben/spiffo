mod config;

use clap::{Parser, Subcommand};

#[derive(Parser, Getters)]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct SpiffoCLI {
    /// Enable debug mode
    #[arg(long, env = "DEBUG", default_value_t = true)]
    debug: bool,

    /// All Spiffo commands
    #[command(subcommand)]
    command: SpiffoCmd,
}

#[derive(Debug, Subcommand)]
pub enum SpiffoCmd {
    /// Prints information about Spiffo
    About,

    /// Configure settings for the server
    Config(config::ConfigArgs),
}

impl SpiffoCmd {
    pub fn execute(&self) {
        use SpiffoCmd::*;
        match self {
            About => crate::messages::about(),
            Config(args) => args.command().execute(),
        }
    }
}
