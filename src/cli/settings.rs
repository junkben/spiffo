use clap::Parser;

macro_rules! settings {
    ($(
        $(#[$($attrs:tt)*])*
        $key:ident: $type:ty = $value:expr
    ),*) => {
        #[derive(Parser, Getters)]
        pub struct Settings {
            $(
                $(#[$($attrs)*])*
                #[arg(long, env = stringify!($name).to_uppercase(), default_value_t = $value)]
                $key: $type,
            ),*
        }
    };
}

settings!(
    /// Players can hurt and kill other players
    pvp: bool = true,
    
    /// Game time stops when there are no players online
    pause_empty: bool = true,
    
    /// Toggles global chat on or off.
    global_chat: bool = true,
    
    chat_streams: String = "s,r,a,w,y,sh,f,all".to_owned(),
    
    /// Clients may join without already having an account in the whitelist. If set to false, administrators must manually create username/password combos.
    open: bool = true,
    
    /// The first welcome message visible in the chat panel. This will be displayed immediately after player login. you can use RGB colours to chance the colour of the welcome message. You can also use <LINE> to create a separate lines within your text. Use: <RGB:1,0,0> This message will show up red!
    server_welcome_message: String = "Welcome to Project Zomboid Multiplayer! <LINE> <LINE> To interact with the Chat panel: press Tab, T, or Enter. <LINE> <LINE> The Tab key will change the target stream of the message. <LINE> <LINE> Global Streams: /all <LINE> Local Streams: /say, /yell <LINE> Special Steams: /whisper, /safehouse, /faction. <LINE> <LINE> Press the Up arrow to cycle through your message history. Click the Gear icon to customize chat. <LINE> <LINE> Happy surviving!".to_owned(),
    
    /// Add unknown usernames to the whitelist when players join. Clients will supply their own username/password on joining. (This is for Open=true servers)
    auto_create_user_in_white_list: bool = false,
    
    /// Display usernames above player's heads in-game.
    display_user_name: bool = true,
    
    /// Display first & last name above player's heads.
    show_first_and_last_name: bool = false,
    
    /// Force every new player to spawn at these set x,y,z world coordinates. Find desired coordinates at map.projectzomboid.com. (Ignored when 0,0,0)
    spawn_point: String = "0,0,0".to_owned(),
    
    /// Players can enter and leave PVP on an individual basis. A player can only hurt another player when at least one of them is in PVP mode - as shown by the unobscured skull and crossbones on the left of the screen. When SafetySystem=false, players are free to hurt each other at any time if PVP is enabled.
    safety_timer: bool = true,
    
    /// Display a skull icon over the head of players who have entered PVP mode
    show_safety: bool = true,
    
    /// The time it takes for a player to enter and leave PVP mode\nMinimum=0 Maximum=1000 Default=2
    safety_toggle_timer: usize = 2,
    
    /// The delay before a player can enter or leave PVP mode again, having recently done so\nMinimum=0 Maximum=1000 Default=3
    safety_cooldown_timer: usize = 3,
    
    /// Item types new players spawn with.\nSeparate multiple item types with commas.\nExample: Base.Axe,Base.Bag_BigHikingBag
    spawn_items: String = "".to_owned(),
    
    /// Default starting port for player data. If UDP, this is this one of two ports used.\nMinimum=0 Maximum=65535 Default=16261
    default_port: usize = 16261,
    
    /// Minimum=0 Maximum=65535 Default=16262
    udp_port: usize = 16262,
    
    /// Reset ID determines if the server has undergone a soft-reset. If this number does match the client, the client must create a new character. Used in conjunction with PlayerServerID. It is strongly advised that you backup these IDs somewhere\nMinimum=0 Maximum=2147483647 Default=895534392
    reset_id: usize = 895534392,
    
    /// Enter the mod loading ID here. It can be found in \Steam\steamapps\workshop\modID\mods\modName\info.txt
    mods: String = "".to_owned(),
    
    /// Enter the foldername of the mod found in \Steam\steamapps\workshop\modID\mods\modName\media\maps\
    map: String = "Muldraugh, KY".to_owned(),
    
    /// Kick clients whose game files don't match the server's.
    do_lua_checksum: bool = true,
    
    deny_login_on_overloaded_server: bool = true,
    
    /// Shows the server on the in-game browser. (Note: Steam-enabled servers are always visible in the Steam server browser)
    public: bool = false,
    
    /// Name of the server displayed in the in-game browser and, if applicable, the Steam browser
    public_name: String = "My PZ Server".to_owned(),
    
    /// Description displayed in the in-game public server browser. Typing \n will create a new line in your description
    public_description: String = "".to_owned(),
    
    /// Maximum number of players that can be on the server at one time. This excludes admins.
    /// WARNING: Server player counts above 32 will potentially result in poor map streaming and desync. Please advance with caution.\nMinimum=1 Maximum=100 Default=32
    max_players: usize = 32,
    
    /// Ping limit, in milliseconds, before a player is kicked from the server. (Set to 100 to disable)\nMinimum=100 Maximum=2147483647 Default=400
    ping_limit: u32 = 400,
    
    /// After X hours, all containers in the world will respawn loot. To spawn loot a container must have been looted at least once. Loot respawn is not impacted by visibility or subsequent looting.\nMinimum=0 Maximum=2147483647 Default=0
    hours_for_loot_respawn: u32 = 0,
    
    /// Containers with a number of items greater, or equal to, this setting will not respawn\nMinimum=1 Maximum=2147483647 Default=4
    MaxItemsForLootRespawn: = 4,
    
    /// Items will not respawn in buildings that players have barricaded or built in
    ConstructionPreventsLootRespawn: bool = true,
    
    /// Remove player accounts from the whitelist after death. This prevents players creating a new character after death on Open=false servers
    DropOffWhiteListAfterDeath: bool = false,
    
    /// All forms of fire are disabled - except for campfires
    NoFire: bool = false,
    
    /// If checked, every time a player dies a global message will be displayed in the chat
    AnnounceDeath: bool = false,
    
    /// The number of in-game minutes it takes to read one page of a book\nMinimum=0.00 Maximum=60.00 Default=1.00
    MinutesPerPage: = 1.0,
    
    /// Loaded parts of the map are saved after this set number of real-world minutes have passed. (The map is usually saved only after clients leave a loaded area)\nMinimum=0 Maximum=2147483647 Default=0
    SaveWorldEveryMinutes: = 0,
    
    /// Both admins and players can claim safehouses
    PlayerSafehouse: bool = false,
    
    /// Only admins can claim safehouses
    AdminSafehouse: bool = false,
    
    /// Allow non-members to enter a safehouse without being invited
    SafehouseAllowTrepass: bool = true,
    
    /// Allow fire to damage safehouses
    SafehouseAllowFire: bool = true,
    
    /// Allow non-members to take items from safehouses
    SafehouseAllowLoot: bool = true,
    
    /// Players will respawn in a safehouse that they were a member of before they died
    SafehouseAllowRespawn: bool = false,
    
    /// Players must have survived this number of in-game days before they are allowed to claim a safehouse\nMinimum=0 Maximum=2147483647 Default=0
    SafehouseDaySurvivedToClaim: = 0,
    
    /// Players are automatically removed from a safehouse they have not visited for this many real-world hours\nMinimum=0 Maximum=2147483647 Default=144
    SafeHouseRemovalTime: = 144,
    
    /// Governs whether players can claim non-residential buildings.
    SafehouseAllowNonResidential: bool = false,
    
    /// Allow players to destroy world objects with sledgehammers
    AllowDestructionBySledgehammer: bool = true,
    
    /// Allow players to destroy world objects only in their safehouse (require AllowDestructionBySledgehammer to true).
    SledgehammerOnlyInSafehouse: bool = false,
    
    /// Kick players that appear to be moving faster than is possible. May be buggy -- use with caution.
    KickFastPlayers: bool = false,
    
    /// ServerPlayerID determines if a character is from another server, or single player. This value may be changed by soft resets. If this number does match the client, the client must create a new character. This is used in conjunction with ResetID. It is strongly advised that you backup these IDs somewhere
    ServerPlayerID: = 1493536879,
    
    /// The port for the RCON (Remote Console)\nMinimum=0 Maximum=65535 Default=27015
    RCONPort: = 27015,
    
    /// RCON password (Pick a strong password)
    RCONPassword: = ,
    
    /// Enables global text chat integration with a Discord channel
    DiscordEnable: bool = false,
    
    /// Discord bot access token
    DiscordToken: = ,
    
    /// The Discord channel name. (Try the separate channel ID option if having difficulties)
    DiscordChannel: = ,
    
    /// The Discord channel ID. (Use if having difficulties with Discord channel name option)
    DiscordChannelID: = ,
    
    /// Clients must know this password to join the server. (Ignored when hosting a server via the Host button)
    Password: = ,
    
    /// Limits the number of different accounts a single Steam user may create on this server. Ignored when using the Hosts button.\nMinimum=0 Maximum=2147483647 Default=0
    MaxAccountsPerUser: = 0,
    
    /// Allow co-op/splitscreen players
    AllowCoop: bool = true,
    
    /// Players are allowed to sleep when their survivor becomes tired, but they do not NEED to sleep
    SleepAllowed: bool = false,
    
    /// Players get tired and need to sleep. (Ignored if SleepAllowed=false)
    SleepNeeded: bool = false,
    
    KnockedDownAllowed: bool = true,
    
    SneakModeHideFromOtherPlayers: bool = true,
    
    /// List Workshop Mod IDs for the server to download. Each must be separated by a semicolon. Example: WorkshopItems=514427485;513111049
    WorkshopItems: = ,
    
    /// Show Steam usernames and avatars in the Players list. Can be true (visible to everyone), false (visible to no one), or admin (visible to only admins)
    SteamScoreboard: bool = true,
    
    /// Enable the Steam VAC system
    SteamVAC: bool = true,
    
    /// Attempt to configure a UPnP-enabled internet gateway to automatically setup port forwarding rules. The server will fall back to default ports if this fails
    UPnP: bool = true,
    
    /// VOIP is enabled when checked
    VoiceEnable: bool = true,
    
    /// The minimum tile distance over which VOIP sounds can be heard.\nMinimum=0.00 Maximum=100000.00 Default=10.00
    VoiceMinDistance: = 10.0,
    
    /// The maximum tile distance over which VOIP sounds can be heard.\nMinimum=0.00 Maximum=100000.00 Default=100.00
    VoiceMaxDistance: = 100.0,
    
    /// Toggle directional audio for VOIP
    Voice3D: bool = true,
    
    /// Minimum=10.00 Maximum=150.00 Default=70.00
    SpeedLimit: = 70.0,
    
    LoginQueueEnabled: bool = false,
    
    /// Minimum=20 Maximum=1200 Default=60
    LoginQueueConnectTimeout: = 60,
    
    /// Set the IP from which the server is broadcast. This is for network configurations with multiple IP addresses, such as server farms
    server_browser_announced_ip: = ,
    
    /// Players can respawn in-game at the coordinates where they died
    PlayerRespawnWithSelf: bool = false,
    
    /// Players can respawn in-game at a split screen / Remote Play player's location
    PlayerRespawnWithOther: bool = false,
    
    /// Governs how fast time passes while players sleep. Value multiplies the speed of the time that passes during sleeping.\nMinimum=1.00 Maximum=100.00 Default=40.00
    FastForwardMultiplier: = 40.0,
    
    /// Safehouse acts like a normal house if a member of the safehouse is connected (so secure when players are offline)
    DisableSafehouseWhenPlayerConnected: bool = false,
    
    /// Players can create factions when true
    Faction: bool = true,
    
    /// Players must survive this number of in-game days before being allowed to create a faction\nMinimum=0 Maximum=2147483647 Default=0
    FactionDaySurvivedToCreate: = 0,
    
    /// Number of players required as faction members before the faction owner can create a group tag\nMinimum=1 Maximum=2147483647 Default=1
    FactionPlayersRequiredForTag: = 1,
    
    /// Disables radio transmissions from players with an access level
    DisableRadioStaff: bool = false,
    
    /// Disables radio transmissions from players with 'admin' access level
    DisableRadioAdmin: bool = true,
    
    /// Disables radio transmissions from players with 'gm' access level
    DisableRadioGM: bool = true,
    
    /// Disables radio transmissions from players with 'overseer' access level
    DisableRadioOverseer: bool = false,
    
    /// Disables radio transmissions from players with 'moderator' access level
    DisableRadioModerator: bool = false,
    
    /// Disables radio transmissions from invisible players
    DisableRadioInvisible: bool = true,
    
    /// Semicolon-separated list of commands that will not be written to the cmd.txt server log. For example: \n-vehicle. Inputting * means do NOT write any vehicle command. Inputting: \n+vehicle.installPart means DO write that command
    ClientCommandFilter: = -vehicle.*;+vehicle.damageWindow;+vehicle.fixPart;+vehicle.installPart;+vehicle.uninstallPart,
    
    /// Semicolon-separated list of actions that will be written to the ClientActionLogs.txt server log.
    ClientActionLogs: = ISEnterVehicle;ISExitVehicle;ISTakeEngineParts;,
    
    /// Track changes in player perk levels in PerkLog.txt server log
    PerkLogs: bool = true,
    
    /// Maximum number of items that can be placed in a container.  Zero means there is no limit. (PLEASE NOTE: This includes individual small items such as nails. A limit of 50 will mean only 50 nails can be stored.)\nMinimum=0 Maximum=9000 Default=0
    ItemNumbersLimitPerContainer: = 0,
    
    /// Number of days before old blood splats are removed.
    /// Removal happens when map chunks are loaded.
    /// Zero means they will never disappear\nMinimum=0 Maximum=365 Default=0
    BloodSplatLifespanDays: = 0,
    
    /// Allow use of non-ASCII (cyrillic etc) characters in usernames
    AllowNonAsciiUsername: bool = false,
    
    BanKickGlobalSound: bool = true,
    
    /// If enabled, when HoursForCorpseRemoval triggers, it will also remove player�s corpses from the ground.
    RemovePlayerCorpsesOnCorpseRemoval: bool = false,
    
    /// If true, player can use the "delete all" button on bins.
    TrashDeleteAll: bool = false,
    
    /// If true, player can hit again when struck by another player.
    PVPMeleeWhileHitReaction: bool = false,
    
    /// If true, players will have to mouse over someone to see their display name.
    MouseOverToSeeDisplayName: bool = true,
    
    /// If true, automatically hide the player you can't see (like zombies).
    HidePlayersBehindYou: bool = true,
    
    /// Damage multiplier for PVP melee attacks.\nMinimum=0.00 Maximum=500.00 Default=30.00
    PVPMeleeDamageModifier: = 30.0,
    
    /// Damage multiplier for PVP ranged attacks.\nMinimum=0.00 Maximum=500.00 Default=50.00
    PVPFirearmDamageModifier: = 50.0,
    
    /// Modify the range of zombie attraction to cars. (Lower values can help with lag.)\nMinimum=0.00 Maximum=10.00 Default=0.50
    CarEngineAttractionModifier: = 0.5,
    
    /// Governs whether players bump (and knock over) other players when running through them.
    PlayerBumpPlayer: bool = false,
    
    /// Controls display of remote players on the in-game map.\n1=Hidden 2=Friends 3=Everyone\nMinimum=1 Maximum=3 Default=1
    MapRemotePlayerVisibility: = 1,
    
    /// Minimum=1 Maximum=300 Default=5
    BackupsCount: = 5,
    
    BackupsOnStart: bool = true,
    
    BackupsOnVersionChange: bool = true,
    
    /// Minimum=0 Maximum=1500 Default=0
    BackupsPeriod: = 0,
    
    /// Disables anti-cheat protection for type 1.
    AntiCheatProtectionType1: bool = true,
    
    /// Disables anti-cheat protection for type 2.
    AntiCheatProtectionType2: bool = true,
    
    /// Disables anti-cheat protection for type 3.
    AntiCheatProtectionType3: bool = true,
    
    /// Disables anti-cheat protection for type 4.
    AntiCheatProtectionType4: bool = true,
    
    /// Disables anti-cheat protection for type 5.
    AntiCheatProtectionType5: bool = true,
    
    /// Disables anti-cheat protection for type 6.
    AntiCheatProtectionType6: bool = true,
    
    /// Disables anti-cheat protection for type 7.
    AntiCheatProtectionType7: bool = true,
    
    /// Disables anti-cheat protection for type 8.
    AntiCheatProtectionType8: bool = true,
    
    /// Disables anti-cheat protection for type 9.
    AntiCheatProtectionType9: bool = true,
    
    /// Disables anti-cheat protection for type 10.
    AntiCheatProtectionType10: bool = true,
    
    /// Disables anti-cheat protection for type 11.
    AntiCheatProtectionType11: bool = true,
    
    /// Disables anti-cheat protection for type 12.
    AntiCheatProtectionType12: bool = true,
    
    /// Disables anti-cheat protection for type 13.
    AntiCheatProtectionType13: bool = true,
    
    /// Disables anti-cheat protection for type 14.
    AntiCheatProtectionType14: bool = true,
    
    /// Disables anti-cheat protection for type 15.
    AntiCheatProtectionType15: bool = true,
    
    /// Disables anti-cheat protection for type 16.
    AntiCheatProtectionType16: bool = true,
    
    /// Disables anti-cheat protection for type 17.
    AntiCheatProtectionType17: bool = true,
    
    /// Disables anti-cheat protection for type 18.
    AntiCheatProtectionType18: bool = true,
    
    /// Disables anti-cheat protection for type 19.
    AntiCheatProtectionType19: bool = true,
    
    /// Disables anti-cheat protection for type 20.
    AntiCheatProtectionType20: bool = true,
    
    AntiCheatProtectionType21: bool = true,
    
    AntiCheatProtectionType22: bool = true,
    
    AntiCheatProtectionType23: bool = true,
    
    AntiCheatProtectionType24: bool = true,
    
    /// Threshold value multiplier for anti-cheat protection: type 2.\nMinimum=1.00 Maximum=10.00 Default=3.00
    AntiCheatProtectionType2ThresholdMultiplier: = 3.0,
    
    /// Threshold value multiplier for anti-cheat protection: type 3.\nMinimum=1.00 Maximum=10.00 Default=1.00
    AntiCheatProtectionType3ThresholdMultiplier: = 1.0,
    
    /// Threshold value multiplier for anti-cheat protection: type 4.\nMinimum=1.00 Maximum=10.00 Default=1.00
    AntiCheatProtectionType4ThresholdMultiplier: = 1.0,
    
    /// Threshold value multiplier for anti-cheat protection: type 9.\nMinimum=1.00 Maximum=10.00 Default=1.00
    AntiCheatProtectionType9ThresholdMultiplier: = 1.0,
    
    /// Threshold value multiplier for anti-cheat protection: type 15.\nMinimum=1.00 Maximum=10.00 Default=1.00
    AntiCheatProtectionType15ThresholdMultiplier: = 1.0,
    
    /// Threshold value multiplier for anti-cheat protection: type 20.\nMinimum=1.00 Maximum=10.00 Default=1.00
    AntiCheatProtectionType20ThresholdMultiplier: = 1.0,
    
    /// Minimum=1.00 Maximum=10.00 Default=1.00
    AntiCheatProtectionType22ThresholdMultiplier: = 1.0,
    
    /// Minimum=1.00 Maximum=10.00 Default=6.00
    AntiCheatProtectionType24ThresholdMultiplier: = 6.0  
);