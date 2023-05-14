use clap::Parser;

//use indexmap::IndexMap;
use serde::{Deserialize, Serialize};


#[derive(Parser, Getters, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    #[command(flatten)]
    #[serde(flatten)]
    discord: DiscordSettings,

    #[command(flatten)]
    #[serde(flatten)]
    voip: VOIPSettings,

    #[command(flatten)]
    #[serde(flatten)]
    anticheat: AntiCheatSettings,

    #[command(flatten)]
    #[serde(flatten)]
    safehouse: SafehouseSettings,

    #[command(flatten)]
    #[serde(flatten)]
    network: NetworkSettings,

    #[command(flatten)]
    #[serde(flatten)]
    transmission: TransmissionSettings,

    #[command(flatten)]
    #[serde(flatten)]
    mods: ModSettings,

    #[command(flatten)]
    #[serde(flatten)]
    chat: ChatSettings,

    #[command(flatten)]
    #[serde(flatten)]
    backups: BackupSettings,

    #[command(flatten)]
    #[serde(flatten)]
    other: OtherSettings,
}

macro_rules! settings {
    (
        $struct_name:ident {
            $(
                $(#[$($attrs:tt)*])*
                $key:ident: $type:ty = $value:expr
            ),*
        }
    ) => {        
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

settings!(
    DiscordSettings {
        /// Enables global text chat integration with a Discord channel
        #[serde(with = "crate::serde::bool")]
        discord_enable: bool = false,
        
        /// Discord bot access token
        discord_token: String = format!(""),
        
        /// The Discord channel name. (Try the separate channel ID option if having difficulties)
        discord_channel: String = format!(""),
        
        /// The Discord channel ID. (Use if having difficulties with Discord channel name option)
        #[serde(alias = "DiscordChannelID")]
        discord_channel_id: String = format!("")
    }
);

settings!(
    VOIPSettings {
        /// VOIP is enabled when checked
        #[serde(with = "crate::serde::bool")]
        voice_enable: bool = true,

        /// The minimum tile distance over which VOIP sounds can be heard.\nMinimum=0.00 Maximum=100000.00 Default=10.00
        #[serde(with = "crate::serde::f32")]
        voice_min_distance: f32 = 10.0,
        
        /// The maximum tile distance over which VOIP sounds can be heard.\nMinimum=0.00 Maximum=100000.00 Default=100.00
        #[serde(with = "crate::serde::f32")]
        voice_max_distance: f32 = 100.0,
        
        /// Toggle directional audio for VOIP
        #[serde(alias = "Voice3D")]
        #[serde(with = "crate::serde::bool")]
        voice_3d: bool = true
    }
);

settings!(
    AntiCheatSettings {
        /// Disables anti-cheat protection for type 1.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_1: bool = true,
        
        /// Disables anti-cheat protection for type 2.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_2: bool = true,
        
        /// Disables anti-cheat protection for type 3.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_3: bool = true,
        
        /// Disables anti-cheat protection for type 4.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_4: bool = true,
        
        /// Disables anti-cheat protection for type 5.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_5: bool = true,
        
        /// Disables anti-cheat protection for type 6.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_6: bool = true,
        
        /// Disables anti-cheat protection for type 7.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_7: bool = true,
        
        /// Disables anti-cheat protection for type 8.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_8: bool = true,
        
        /// Disables anti-cheat protection for type 9.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_9: bool = true,
        
        /// Disables anti-cheat protection for type 10.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_10: bool = true,
        
        /// Disables anti-cheat protection for type 11.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_11: bool = true,
        
        /// Disables anti-cheat protection for type 12.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_12: bool = true,
        
        /// Disables anti-cheat protection for type 13.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_13: bool = true,
        
        /// Disables anti-cheat protection for type 14.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_14: bool = true,
        
        /// Disables anti-cheat protection for type 15.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_15: bool = true,
        
        /// Disables anti-cheat protection for type 16.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_16: bool = true,
        
        /// Disables anti-cheat protection for type 17.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_17: bool = true,
        
        /// Disables anti-cheat protection for type 18.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_18: bool = true,
        
        /// Disables anti-cheat protection for type 19.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_19: bool = true,
        
        /// Disables anti-cheat protection for type 20.
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_20: bool = true,
        
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_21: bool = true,
        
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_22: bool = true,
        
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_23: bool = true,
        
        #[serde(with = "crate::serde::bool")]
        anti_cheat_protection_type_24: bool = true,
        
        /// Threshold value multiplier for anti-cheat protection: type 2.\nMinimum=1.00 Maximum=10.00 Default=3.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_2_threshold_multiplier: f32 = 3.0,
        
        /// Threshold value multiplier for anti-cheat protection: type 3.\nMinimum=1.00 Maximum=10.00 Default=1.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_3_threshold_multiplier: f32 = 1.0,
        
        /// Threshold value multiplier for anti-cheat protection: type 4.\nMinimum=1.00 Maximum=10.00 Default=1.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_4_threshold_multiplier: f32 = 1.0,
        
        /// Threshold value multiplier for anti-cheat protection: type 9.\nMinimum=1.00 Maximum=10.00 Default=1.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_9_threshold_multiplier: f32 = 1.0,
        
        /// Threshold value multiplier for anti-cheat protection: type 15.\nMinimum=1.00 Maximum=10.00 Default=1.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_15_threshold_multiplier: f32 = 1.0,
        
        /// Threshold value multiplier for anti-cheat protection: type 20.\nMinimum=1.00 Maximum=10.00 Default=1.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_20_threshold_multiplier: f32 = 1.0,
        
        /// Minimum=1.00 Maximum=10.00 Default=1.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_22_threshold_multiplier: f32 = 1.0,
        
        /// Minimum=1.00 Maximum=10.00 Default=6.00
        #[serde(with = "crate::serde::f32")]
        anti_cheat_protection_type_24_threshold_multiplier: f32 = 6.0
    }
);

settings!(
    SafehouseSettings {
        /// Both admins and players can claim safehouses
        #[serde(with = "crate::serde::bool")]
        player_safehouse: bool = false,

        /// Only admins can claim safehouses
        #[serde(with = "crate::serde::bool")]
        admin_safehouse: bool = false,
        
        /// Allow non-members to enter a safehouse without being invited
        #[serde(alias = "SafehouseAllowTrepass")]
        #[serde(with = "crate::serde::bool")]
        safehouse_allow_trespass: bool = true,
        
        /// Allow fire to damage safehouses
        #[serde(with = "crate::serde::bool")]
        safehouse_allow_fire: bool = true,
        
        /// Allow non-members to take items from safehouses
        #[serde(with = "crate::serde::bool")]
        safehouse_allow_loot: bool = true,
        
        /// Players will respawn in a safehouse that they were a member of before they died
        #[serde(with = "crate::serde::bool")]
        safehouse_allow_respawn: bool = false,
        
        /// Players must have survived this number of in-game days before they are allowed to claim a safehouse\nMinimum=0 Maximum=2147483647 Default=0
        #[serde(with = "crate::serde::u32")]
        safehouse_day_survived_to_claim: u32 = 0,
        
        /// Players are automatically removed from a safehouse they have not visited for this many real-world hours\nMinimum=0 Maximum=2147483647 Default=144
        #[serde(alias = "SafeHouseRemovalTime")]
        #[serde(with = "crate::serde::u32")]
        safehouse_removal_time: u32 = 144,
        
        /// Governs whether players can claim non-residential buildings.
        #[serde(with = "crate::serde::bool")]
        safehouse_allow_non_residential: bool = false,

        /// Safehouse acts like a normal house if a member of the safehouse is connected (so secure when players are offline)
        #[serde(with = "crate::serde::bool")]
        disable_safehouse_when_player_connected: bool = false
    }
);

settings!(
    NetworkSettings {
        /// Default starting port for player data. If UDP, this is this one of two ports used.\nMinimum=0 Maximum=65535 Default=16261
        #[serde(with = "crate::serde::u16")]
        default_port: u16 = 16261,
        
        /// Minimum=0 Maximum=65535 Default=16262
        #[serde(alias = "UDPPort")]
        #[serde(with = "crate::serde::u16")]
        udp_port: u16 = 16262,
        
        /// Reset ID determines if the server has undergone a soft-reset. If this number does match the client, the client must create a new character. Used in conjunction with PlayerServerID. It is strongly advised that you backup these IDs somewhere\nMinimum=0 Maximum=2147483647 Default=895534392
        #[serde(alias = "ResetID")]
        #[serde(with = "crate::serde::u32")]
        reset_id: u32 = 895534392,
        
        /// Kick clients whose game files don't match the server's.
        #[serde(with = "crate::serde::bool")]
        do_lua_checksum: bool = true,
        
        #[serde(with = "crate::serde::bool")]
        deny_login_on_overloaded_server: bool = true,
        
        /// Shows the server on the in-game browser. (Note: Steam-enabled servers are always visible in the Steam server browser)
        #[serde(with = "crate::serde::bool")]
        public: bool = false,
        
        /// Name of the server displayed in the in-game browser and, if applicable, the Steam browser
        public_name: String = format!("My PZ Server"),
        
        /// Description displayed in the in-game public server browser. Typing \n will create a new line in your description
        public_description: String = format!(""),
        
        /// Maximum number of players that can be on the server at one time. This excludes admins.
        /// WARNING: Server player counts above 32 will potentially result in poor map streaming and desync. Please advance with caution.\nMinimum=1 Maximum=100 Default=32
        #[serde(with = "crate::serde::u32")]
        max_players: u32 = 32,
        
        /// Ping limit, in milliseconds, before a player is kicked from the server. (Set to 100 to disable)\nMinimum=100 Maximum=2147483647 Default=400
        #[serde(with = "crate::serde::u32")]
        ping_limit: u32 = 400,

        /// Clients must know this password to join the server. (Ignored when hosting a server via the Host button)
        password: String = format!(""),

        /// Attempt to configure a UPnP-enabled internet gateway to automatically setup port forwarding rules. The server will fall back to default ports if this fails
        #[serde(alias = "UPnP")]
        #[serde(with = "crate::serde::bool")]
        upnp: bool = true,

        #[serde(with = "crate::serde::bool")]
        login_queue_enabled: bool = false,
        
        /// Minimum=20 Maximum=1200 Default=60
        #[serde(with = "crate::serde::u32")]
        login_queue_connect_timeout: u32 = 60,
        
        /// Set the IP from which the server is broadcast. This is for network configurations with multiple IP addresses, such as server farms
        #[serde(alias = "server_browser_announced_ip")]
        server_browser_announced_ip: String = format!(""),

        /// Show Steam usernames and avatars in the Players list. Can be true (visible to everyone), false (visible to no one), or admin (visible to only admins)
        #[serde(with = "crate::serde::bool")]
        steam_scoreboard: bool = true,
        
        /// Enable the Steam VAC system
        #[serde(alias = "SteamVAC")]
        #[serde(with = "crate::serde::bool")]
        steam_vac: bool = true
    }
);

settings!(
    TransmissionSettings {
        /// Disables radio transmissions from players with an access level
        #[serde(with = "crate::serde::bool")]
        disable_radio_staff: bool = false,

        /// Disables radio transmissions from players with 'admin' access level
        #[serde(with = "crate::serde::bool")]
        disable_radio_admin: bool = true,
        
        /// Disables radio transmissions from players with 'gm' access level
        #[serde(alias = "DisableRadioGM")]
        #[serde(with = "crate::serde::bool")]
        disable_radio_gm: bool = true,
        
        /// Disables radio transmissions from players with 'overseer' access level
        #[serde(with = "crate::serde::bool")]
        disable_radio_overseer: bool = false,
        
        /// Disables radio transmissions from players with 'moderator' access level
        #[serde(with = "crate::serde::bool")]
        disable_radio_moderator: bool = false,
        
        /// Disables radio transmissions from invisible players
        #[serde(with = "crate::serde::bool")]
        disable_radio_invisible: bool = true
    }
);

settings!(
    ModSettings {
        /// Enter the mod loading ID here. It can be found in \Steam\steamapps\workshop\modID\mods\modName\info.txt
        mods: String = format!(""),
        
        /// Enter the foldername of the mod found in \Steam\steamapps\workshop\modID\mods\modName\media\maps\
        map: String = format!("Muldraugh, KY"),

        /// List Workshop Mod IDs for the server to download. Each must be separated by a semicolon. Example: WorkshopItems=514427485;513111049
        workshop_items: String = format!("")
    }
);

settings!(
    ChatSettings {
        /// Toggles global chat on or off.
        #[serde(with = "crate::serde::bool")]
        global_chat: bool = true,

        chat_streams: String = format!("s,r,a,w,y,sh,f,all"),

        /// The first welcome message visible in the chat panel. This will be displayed immediately after player login. you can use RGB colours to chance the colour of the welcome message. You can also use <LINE> to create a separate lines within your text. Use: <RGB:1,0,0> This message will show up red!
        server_welcome_message: String = format!("Welcome to Project Zomboid Multiplayer! <LINE> <LINE> To interact with the Chat panel: press Tab, T, or Enter. <LINE> <LINE> The Tab key will change the target stream of the message. <LINE> <LINE> Global Streams: /all <LINE> Local Streams: /say, /yell <LINE> Special Steams: /whisper, /safehouse, /faction. <LINE> <LINE> Press the Up arrow to cycle through your message history. Click the Gear icon to customize chat. <LINE> <LINE> Happy surviving!"),
    
        /// If checked, every time a player dies a global message will be displayed in the chat
        #[serde(with = "crate::serde::bool")]
        announce_death: bool = false
    }
);

settings!(
    BackupSettings {
        /// Minimum=1 Maximum=300 Default=5
        #[serde(with = "crate::serde::u32")]
        backups_count: u32 = 5,
        
        #[serde(with = "crate::serde::bool")]
        backups_on_start: bool = true,
        
        #[serde(with = "crate::serde::bool")]
        backups_on_version_change: bool = true,
        
        /// Minimum=0 Maximum=1500 Default=0
        #[serde(with = "crate::serde::u32")]
        backups_period: u32 = 0
    }
);

settings!(
    OtherSettings {
        /// Players can hurt and kill other players
        #[serde(alias = "PVP")]
        #[serde(with = "crate::serde::bool")]
        pvp: bool = true,
        
        /// Game time stops when there are no players online
        #[serde(with = "crate::serde::bool")]
        pause_empty: bool = true,
        
        /// Clients may join without already having an account in the whitelist. If set to false, administrators must manually create username/password combos.
        #[serde(with = "crate::serde::bool")]
        open: bool = true,
        
        /// Add unknown usernames to the whitelist when players join. Clients will supply their own username/password on joining. (This is for Open=true servers)
        #[serde(with = "crate::serde::bool")]
        auto_create_user_in_white_list: bool = false,
        
        /// Display usernames above player's heads in-game.
        #[serde(with = "crate::serde::bool")]
        display_user_name: bool = true,
        
        /// Display first & last name above player's heads.
        #[serde(with = "crate::serde::bool")]
        show_first_and_last_name: bool = false,
        
        /// Force every new player to spawn at these set x,y,z world coordinates. Find desired coordinates at map.projectzomboid.com. (Ignored when 0,0,0)
        spawn_point: String = format!("0,0,0"),
        
        /// Players can enter and leave PVP on an individual basis. A player can only hurt another player when at least one of them is in PVP mode - as shown by the unobscured skull and crossbones on the left of the screen. When SafetySystem=false, players are free to hurt each other at any time if PVP is enabled.
        #[serde(with = "crate::serde::bool")]
        safety_system: bool = true,
        
        /// Display a skull icon over the head of players who have entered PVP mode
        #[serde(with = "crate::serde::bool")]
        show_safety: bool = true,
        
        /// The time it takes for a player to enter and leave PVP mode\nMinimum=0 Maximum=1000 Default=2
        #[serde(with = "crate::serde::u32")]
        safety_toggle_timer: u32 = 2,
        
        /// The delay before a player can enter or leave PVP mode again, having recently done so\nMinimum=0 Maximum=1000 Default=3
        #[serde(with = "crate::serde::u32")]
        safety_cooldown_timer: u32 = 3,
        
        /// Item types new players spawn with.\nSeparate multiple item types with commas.\nExample: Base.Axe,Base.Bag_BigHikingBag
        spawn_items: String = format!(""),
        
        /// After X hours, all containers in the world will respawn loot. To spawn loot a container must have been looted at least once. Loot respawn is not impacted by visibility or subsequent looting.\nMinimum=0 Maximum=2147483647 Default=0
        #[serde(with = "crate::serde::u32")]
        hours_for_loot_respawn: u32 = 0,
        
        /// Containers with a number of items greater, or equal to, this setting will not respawn\nMinimum=1 Maximum=2147483647 Default=4
        #[serde(with = "crate::serde::u32")]
        max_items_for_loot_respawn: u32 = 4,
        
        /// Items will not respawn in buildings that players have barricaded or built in
        #[serde(with = "crate::serde::bool")]
        construction_prevents_loot_respawn: bool = true,
        
        /// Remove player accounts from the whitelist after death. This prevents players creating a new character after death on Open=false servers
        #[serde(with = "crate::serde::bool")]
        drop_off_white_list_after_death: bool = false,
        
        /// All forms of fire are disabled - except for campfires
        #[serde(with = "crate::serde::bool")]
        no_fire: bool = false,
        
        /// The number of in-game minutes it takes to read one page of a book\nMinimum=0.00 Maximum=60.00 Default=1.00
        #[serde(with = "crate::serde::f32")]
        minutes_per_page: f32 = 1.0,
        
        /// Loaded parts of the map are saved after this set number of real-world minutes have passed. (The map is usually saved only after clients leave a loaded area)\nMinimum=0 Maximum=2147483647 Default=0
        #[serde(with = "crate::serde::u32")]
        save_world_every_minutes: u32 = 0,
        
        /// Allow players to destroy world objects with sledgehammers
        #[serde(with = "crate::serde::bool")]
        allow_destruction_by_sledgehammer: bool = true,
        
        /// Allow players to destroy world objects only in their safehouse (require AllowDestructionBySledgehammer to true).
        #[serde(with = "crate::serde::bool")]
        sledgehammer_only_in_safehouse: bool = false,
        
        /// Kick players that appear to be moving faster than is possible. May be buggy -- use with caution.
        #[serde(with = "crate::serde::bool")]
        kick_fast_players: bool = false,
        
        /// ServerPlayerID determines if a character is from another server, or single player. This value may be changed by soft resets. If this number does match the client, the client must create a new character. This is used in conjunction with ResetID. It is strongly advised that you backup these IDs somewhere
        #[serde(alias = "ServerPlayerID")]
        #[serde(with = "crate::serde::u32")]
        server_player_id: u32 = 1493536879,
        
        /// The port for the RCON (Remote Console)\nMinimum=0 Maximum=65535 Default=27015
        #[serde(alias = "RCONPort")]
        #[serde(with = "crate::serde::u16")]
        rcon_port: u16 = 27015,
        
        /// RCON password (Pick a strong password)
        #[serde(alias = "RCONPassword")]
        rcon_password: String = format!(""),
        
        /// Limits the number of different accounts a single Steam user may create on this server. Ignored when using the Hosts button.\nMinimum=0 Maximum=2147483647 Default=0
        #[serde(with = "crate::serde::u32")]
        max_accounts_per_user: u32 = 0,
        
        /// Allow co-op/splitscreen players
        #[serde(with = "crate::serde::bool")]
        allow_coop: bool = true,
        
        /// Players are allowed to sleep when their survivor becomes tired, but they do not NEED to sleep
        #[serde(with = "crate::serde::bool")]
        sleep_allowed: bool = false,
        
        /// Players get tired and need to sleep. (Ignored if SleepAllowed=false)
        #[serde(with = "crate::serde::bool")]
        sleep_needed: bool = false,
        
        #[serde(with = "crate::serde::bool")]
        knocked_down_allowed: bool = true,
        
        #[serde(with = "crate::serde::bool")]
        sneak_mode_hide_from_other_players: bool = true,
        
        /// Minimum=10.00 Maximum=150.00 Default=70.00
        #[serde(with = "crate::serde::f32")]
        speed_limit: f32 = 70.0,
        
        /// Players can respawn in-game at the coordinates where they died
        #[serde(with = "crate::serde::bool")]
        player_respawn_with_self: bool = false,
        
        /// Players can respawn in-game at a split screen / Remote Play player's location
        #[serde(with = "crate::serde::bool")]
        player_respawn_with_other: bool = false,
        
        /// Governs how fast time passes while players sleep. Value multiplies the speed of the time that passes during sleeping.\nMinimum=1.00 Maximum=100.00 Default=40.00
        #[serde(with = "crate::serde::f32")]
        fast_forward_multiplier: f32 = 40.0,
        
        /// Players can create factions when true
        #[serde(with = "crate::serde::bool")]
        faction: bool = true,
        
        /// Players must survive this number of in-game days before being allowed to create a faction\nMinimum=0 Maximum=2147483647 Default=0
        #[serde(with = "crate::serde::u32")]
        faction_day_survived_to_create: u32 = 0,
        
        /// Number of players required as faction members before the faction owner can create a group tag\nMinimum=1 Maximum=2147483647 Default=1
        #[serde(with = "crate::serde::u32")]
        faction_players_required_for_tag: u32 = 1,
        
        /// Semicolon-separated list of commands that will not be written to the cmd.txt server log. For example: \n-vehicle. Inputting * means do NOT write any vehicle command. Inputting: \n+vehicle.installPart means DO write that command
        client_command_filter: String = format!("-vehicle.*;+vehicle.damageWindow;+vehicle.fixPart;+vehicle.installPart;+vehicle.uninstallPart"),
        
        /// Semicolon-separated list of actions that will be written to the ClientActionLogs.txt server log.
        client_action_logs: String = format!("ISEnterVehicle;ISExitVehicle;ISTakeEngineParts;"),
        
        /// Track changes in player perk levels in PerkLog.txt server log
        #[serde(with = "crate::serde::bool")]
        perk_logs: bool = true,
        
        /// Maximum number of items that can be placed in a container.  Zero means there is no limit. (PLEASE NOTE: This includes individual small items such as nails. A limit of 50 will mean only 50 nails can be stored.)\nMinimum=0 Maximum=9000 Default=0
        #[serde(with = "crate::serde::u32")]
        item_numbers_limit_per_container: u32 = 0,
        
        /// Number of days before old blood splats are removed.
        /// Removal happens when map chunks are loaded.
        /// Zero means they will never disappear\nMinimum=0 Maximum=365 Default=0
        #[serde(with = "crate::serde::u32")]
        blood_splat_lifespan_days: u32 = 0,
        
        /// Allow use of non-ASCII (cyrillic etc) characters in usernames
        #[serde(with = "crate::serde::bool")]
        allow_non_ascii_username: bool = false,
        
        #[serde(with = "crate::serde::bool")]
        ban_kick_global_sound: bool = true,
        
        /// If enabled, when HoursForCorpseRemoval triggers, it will also remove player�s corpses from the ground.
        #[serde(with = "crate::serde::bool")]
        remove_player_corpses_on_corpse_removal: bool = false,
        
        /// If true, player can use the "delete all" button on bins.
        #[serde(with = "crate::serde::bool")]
        trash_delete_all: bool = false,
        
        /// If true, player can hit again when struck by another player.
        #[serde(alias = "PVPMeleeWhileHitReaction")]
        #[serde(with = "crate::serde::bool")]
        pvp_melee_while_hit_reaction: bool = false,
        
        /// If true, players will have to mouse over someone to see their display name.
        #[serde(with = "crate::serde::bool")]
        mouse_over_to_see_display_name: bool = true,
        
        /// If true, automatically hide the player you can't see (like zombies).
        #[serde(with = "crate::serde::bool")]
        hide_players_behind_you: bool = true,
        
        /// Damage multiplier for PVP melee attacks.\nMinimum=0.00 Maximum=500.00 Default=30.00
        #[serde(alias = "PVPMeleeDamageModifier")]
        #[serde(with = "crate::serde::f32")]
        pvp_melee_damage_modifier: f32 = 30.0,
        
        /// Damage multiplier for PVP ranged attacks.\nMinimum=0.00 Maximum=500.00 Default=50.00
        #[serde(with = "crate::serde::f32")]
        #[serde(alias = "PVPFirearmDamageModifier")]
        pvp_firearm_damage_modifier: f32 = 50.0,
        
        /// Modify the range of zombie attraction to cars. (Lower values can help with lag.)\nMinimum=0.00 Maximum=10.00 Default=0.50
        #[serde(with = "crate::serde::f32")]
        car_engine_attraction_modifier: f32 = 0.5,
        
        /// Governs whether players bump (and knock over) other players when running through them.
        #[serde(with = "crate::serde::bool")]
        player_bump_player: bool = false,
        
        /// Controls display of remote players on the in-game map.\n1=Hidden 2=Friends 3=Everyone\nMinimum=1 Maximum=3 Default=1
        #[serde(with = "crate::serde::u32")]
        map_remote_player_visibility: u32 = 1
    }
);
