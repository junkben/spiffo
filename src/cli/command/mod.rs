mod configure;

use clap::Subcommand;

use super::Settings;

#[derive(Subcommand)]
pub enum Command {
    /// Prints information about Spiffo
    About,

    /// Configure settings for the server
    Configure(Settings),
}

impl Command {
    pub fn execute(&self) {
        use Command::*;
        match self {
            About => crate::messages::about(),
            Configure(_) => todo!(),
        }
    }
}