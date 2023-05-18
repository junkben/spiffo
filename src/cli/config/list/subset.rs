use std::path::Path;

use anyhow::Result;
use clap::ValueEnum;

use crate::settings::Settings;

#[derive(Debug, Clone, Eq, PartialEq, ValueEnum)]
pub enum ListSubsets {
    AntiCheat,
    Backup,
    Chat,
    Discord,
    Faction,
    Latency,
    Mods,
    Network,
    Player,
    Pvp,
    Safehouse,
    ServerLog,
    Transmission,
    User,
    Voip,
    World,
}

impl ListSubsets {
    pub fn execute(&self, path: impl AsRef<Path>) -> Result<()> {
        let settings: Settings = crate::fs::read_settings_from_config(path)?;

        use ListSubsets::*;
        let to_print = match self {
            AntiCheat => format!("{}", settings.anticheat()),
            Backup => format!("{}", settings.backups()),
            Chat => format!("{}", settings.chat()),
            Discord => format!("{}", settings.discord()),
            Faction => format!("{}", settings.faction()),
            Latency => format!("{}", settings.latency()),
            Mods => format!("{}", settings.mods()),
            Network => format!("{}", settings.network()),
            Player => format!("{}", settings.player()),
            Pvp => format!("{}", settings.pvp()),
            Safehouse => format!("{}", settings.safehouse()),
            ServerLog => format!("{}", settings.server_log()),
            Transmission => format!("{}", settings.transmission()),
            User => format!("{}", settings.user()),
            Voip => format!("{}", settings.voip()),
            World => format!("{}", settings.world()),
        };

        info!("{}", to_print);
        Ok(())
    }
}
