use std::fmt;

use anyhow::Context;
use clap::Parser;
use indexmap::IndexMap;
use serde::{ser, Deserialize, Serialize};
use serde_json::to_string_pretty;

mod anticheat;
mod backup;
mod chat;
mod discord;
mod faction;
mod latency;
mod mods;
mod network;
mod player;
mod pvp;
mod safehouse;
mod server_log;
mod transmission;
mod user;
mod voip;
mod world;

use crate::fs::{ReadFromFile, WriteToFile};

#[derive(Deserialize, Serialize, Parser, Getters, Debug, Default)]
pub struct Settings {
    #[command(flatten)]
    #[serde(flatten)]
    anticheat: anticheat::AntiCheatSettings,

    #[command(flatten)]
    #[serde(flatten)]
    backups: backup::BackupSettings,

    #[command(flatten)]
    #[serde(flatten)]
    chat: chat::ChatSettings,

    #[command(flatten)]
    #[serde(flatten)]
    discord: discord::DiscordSettings,

    #[command(flatten)]
    #[serde(flatten)]
    faction: faction::FactionSettings,

    #[command(flatten)]
    #[serde(flatten)]
    latency: latency::LatencySettings,

    #[command(flatten)]
    #[serde(flatten)]
    mods: mods::ModSettings,

    #[command(flatten)]
    #[serde(flatten)]
    network: network::NetworkSettings,

    #[command(flatten)]
    #[serde(flatten)]
    player: player::PlayerSettings,

    #[command(flatten)]
    #[serde(flatten)]
    pvp: pvp::PvpSettings,

    #[command(flatten)]
    #[serde(flatten)]
    safehouse: safehouse::SafehouseSettings,

    #[command(flatten)]
    #[serde(flatten)]
    server_log: server_log::ServerLogSettings,

    #[command(flatten)]
    #[serde(flatten)]
    transmission: transmission::TransmissionSettings,

    #[command(flatten)]
    #[serde(flatten)]
    user: user::UserSettings,

    #[command(flatten)]
    #[serde(flatten)]
    voip: voip::VOIPSettings,

    #[command(flatten)]
    #[serde(flatten)]
    world: world::WorldSettings
}

impl WriteToFile for Settings {
    fn write_to_path(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        fn as_indexmap<T>(subsetting: T) -> IndexMap<String, String>
        where
            T: Sized + ser::Serialize
        {
            let settings_str = serde_json::to_string(&subsetting).unwrap();
            serde_json::from_str::<IndexMap<String, String>>(&settings_str).unwrap()
        }

        let index_maps = [
            as_indexmap(self.anticheat()),
            as_indexmap(self.backups()),
            as_indexmap(self.chat()),
            as_indexmap(self.discord()),
            as_indexmap(self.faction()),
            as_indexmap(self.latency()),
            as_indexmap(self.mods()),
            as_indexmap(self.network()),
            as_indexmap(self.player()),
            as_indexmap(self.pvp()),
            as_indexmap(self.safehouse()),
            as_indexmap(self.server_log()),
            as_indexmap(self.transmission()),
            as_indexmap(self.user()),
            as_indexmap(self.voip()),
            as_indexmap(self.world())
        ];

        let contents = index_maps
            .into_iter()
            .flat_map(|imap| {
                imap.into_iter()
                    .map(|(k, v)| [k, v].join("="))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .join("\r\n");

        debug!("Writing on path {:?}", &path.as_ref());

        std::fs::write(&path, contents).context("failed filesystem write")?;

        debug!("Successfully wrote contents to path");
        Ok(())
    }
}

impl ReadFromFile for Settings {
    fn read_from_file(file_path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        debug!("Reading on path {:?}", &file_path.as_ref());

        let mut bytes: Vec<u8> = std::fs::read(&file_path)?;

        // Need to verify that character 12305 gets changed to an apostrophe
        bytes[12305] = 39;

        let input: String = String::from_utf8(bytes).unwrap();

        // Make a new case-sensitive Ini object to store the data within
        let mut ini = configparser::ini::Ini::new_cs();

        // Exclude ; from the delimiters
        ini.set_comment_symbols(&['#']);

        let index_map_str_im = ini.read(input).expect("config load failed");

        let index_map_str_opt = index_map_str_im
            .get("default")
            .expect("no \"default\" section")
            .clone();

        let index_map_str_str = index_map_str_opt
            .into_iter()
            .map(|(k, v)| (k, v.unwrap()))
            .collect::<indexmap::IndexMap<String, String>>();

        let settings_str = serde_json::to_string(&index_map_str_str).unwrap();

        let settings: Settings = serde_json::from_str(&settings_str).unwrap();
        Ok(settings)
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            to_string_pretty(self).expect("failed serde_json conversion")
        )
    }
}

#[macro_export]
macro_rules! settings {
    (
        $struct_name:ident {
            $(
                $(#[$($attrs:tt)*])*
                $key:ident: $type:ty = $value:expr
            ),*
        }
    ) => {
        use serde::{Deserialize, Serialize};
        use clap::Parser;

        #[derive(Parser, Getters, Clone, Debug, Deserialize, Serialize)]
        #[serde(rename_all = "PascalCase")]
        pub struct $struct_name {
            $(
                $(#[$($attrs)*])*
                #[arg(long, /*env = &stringify!($key).to_uppercase(),*/ default_value_t = $value)]
                $key: $type
            ),*
        }

        impl Default for $struct_name {
            fn default() -> $struct_name {
                $struct_name {
                    $(
                        $key: $value
                    ),*
                }
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", serde_json::to_string_pretty(self).expect("failed serde_json conversion"))
            }
        }
    };
}
