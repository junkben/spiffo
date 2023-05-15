use clap::{Args, Subcommand};

#[derive(Debug, Args, Getters)]
pub struct ConfigArgs  {
    /// Absolute path to the Zomboid Server directory
    #[arg(short, long, env = "SERVER_DIR", default_value_t = format!("~/Zomboid/Server"))]
    directory: String,

    /// Name of the server ini config file
    #[arg(short, long, env = "SERVER_FILENAME", default_value_t = format!("servertest.ini"))]
    filename: String,

    #[command(subcommand)]
    command: ConfigCommands,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    /// Retrieve a config value from the key
    Get {
        /// The key of the config
        key: String,
    },
    /// Set the value of a config 
    Set {
        /// The key of the config
        key: String,
        /// The new value to write to the config file
        value: String,
    },
    /// List all of the config keys and their current values
    List,
}

impl ConfigCommands {
    pub fn execute(&self) {
        use ConfigCommands::*;
        match self {
            Get { .. } => todo!(),
            Set { .. } => todo!(),
            List => todo!(),
        }
    }
}