mod about;
mod config;
mod start;

use anyhow::Context;
use clap::{Parser, Subcommand};

#[derive(Parser, Getters)]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct SpiffoCLI {
    /// All Spiffo commands
    #[command(subcommand)]
    command: SpiffoCmd,

    /// Enable debug mode
    #[arg(long, env = "DEBUG", default_value_t = true)]
    debug: bool
}

#[derive(Debug, Subcommand)]
pub enum SpiffoCmd {
    /// Prints information about Spiffo
    About,

    /// Configure settings for the server
    Config(config::ConfigArgs),

    Start
}

impl SpiffoCmd {
    pub fn execute(&self) -> anyhow::Result<()> {
        use SpiffoCmd::*;
        match self {
            About => about::cmd().context("about cmd failed"),
            Config(args) => args.execute(),
            Start => start::cmd().context("start cmd failed")
        }
    }
}
