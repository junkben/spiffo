use clap::Parser;
use serde::{Deserialize, Serialize};

use indexmap::IndexMap;

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

pub use {
    anticheat::*, backup::*, chat::*, discord::*, faction::*, latency::*, mods::*, network::*,
    player::*, pvp::*, safehouse::*, server_log::*, transmission::*, user::*, voip::*, world::*,
};

#[derive(Parser, Getters, Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    #[command(flatten)]
    #[serde(flatten)]
    anticheat: AntiCheatSettings,

    #[command(flatten)]
    #[serde(flatten)]
    backups: BackupSettings,

    #[command(flatten)]
    #[serde(flatten)]
    chat: ChatSettings,

    #[command(flatten)]
    #[serde(flatten)]
    discord: DiscordSettings,

    #[command(flatten)]
    #[serde(flatten)]
    faction: FactionSettings,

    #[command(flatten)]
    #[serde(flatten)]
    latency: LatencySettings,

    #[command(flatten)]
    #[serde(flatten)]
    mods: ModSettings,

    #[command(flatten)]
    #[serde(flatten)]
    network: NetworkSettings,

    #[command(flatten)]
    #[serde(flatten)]
    player: PlayerSettings,

    #[command(flatten)]
    #[serde(flatten)]
    pvp: PvpSettings,

    #[command(flatten)]
    #[serde(flatten)]
    safehouse: SafehouseSettings,

    #[command(flatten)]
    #[serde(flatten)]
    server_log: ServerLogSettings,

    #[command(flatten)]
    #[serde(flatten)]
    transmission: TransmissionSettings,

    #[command(flatten)]
    #[serde(flatten)]
    user: UserSettings,

    #[command(flatten)]
    #[serde(flatten)]
    voip: VOIPSettings,

    #[command(flatten)]
    #[serde(flatten)]
    world: WorldSettings,
}

impl From<IndexMap<String, String>> for Settings {
    fn from(map: IndexMap<String, String>) -> Settings {
        let map_str = serde_json::to_string(&map).unwrap();
        serde_json::from_str(&map_str).unwrap()
    }
}

impl Into<IndexMap<String, String>> for Settings {
    fn into(self) -> IndexMap<String, String> {
        let settings_str = serde_json::to_string(&self).unwrap();
        serde_json::from_str(&settings_str).unwrap()
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

        #[derive(Parser, Getters, Serialize, Deserialize, Clone, Debug)]
        #[serde(default, rename_all = "PascalCase")]
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
    };
}
