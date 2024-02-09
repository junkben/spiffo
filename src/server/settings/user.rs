crate::settings!(UserSettings {
    /// Clients may join without already having an account in the whitelist. If
    /// set to false, administrators must manually create username/password
    /// combos.
    #[serde(with = "crate::serde::bool")]
    open: bool = true,

    /// Add unknown usernames to the whitelist when players join. Clients will
    /// supply their own username/password on joining. (This is for Open=true
    /// servers)
    #[serde(with = "crate::serde::bool")]
    auto_create_user_in_white_list: bool = false,

    /// Display usernames above player's heads in-game.
    #[serde(with = "crate::serde::bool")]
    display_user_name: bool = true,

    /// Display first & last name above player's heads.
    #[serde(with = "crate::serde::bool")]
    show_first_and_last_name: bool = false,

    /// Force every new player to spawn at these set x,y,z world coordinates.
    /// Find desired coordinates at map.projectzomboid.com. (Ignored when
    /// 0,0,0)
    spawn_point: String = format!("0,0,0"),

    /// Item types new players spawn with.\nSeparate multiple item types with
    /// commas.\nExample: Base.Axe,Base.Bag_BigHikingBag
    spawn_items: String = format!(""),

    /// Remove player accounts from the whitelist after death. This prevents
    /// players creating a new character after death on Open=false servers
    #[serde(with = "crate::serde::bool")]
    drop_off_white_list_after_death: bool = false,

    /// ServerPlayerID determines if a character is from another server, or
    /// single player. This value may be changed by soft resets. If this number
    /// does match the client, the client must create a new character. This is
    /// used in conjunction with ResetID. It is strongly advised that you
    /// backup these IDs somewhere
    #[serde(rename = "ServerPlayerID")]
    #[serde(with = "crate::serde::u32")]
    server_player_id: u32 = 1493536879,

    /// Limits the number of different accounts a single Steam user may create
    /// on this server. Ignored when using the Hosts button.\nMinimum=0
    /// Maximum=2147483647 Default=0
    #[serde(with = "crate::serde::u32")]
    max_accounts_per_user: u32 = 0,

    /// Allow co-op/splitscreen players
    #[serde(with = "crate::serde::bool")]
    allow_coop: bool = true,

    /// Allow use of non-ASCII (cyrillic etc) characters in usernames
    #[serde(with = "crate::serde::bool")]
    allow_non_ascii_username: bool = false,

    ///
    #[serde(with = "crate::serde::bool")]
    ban_kick_global_sound: bool = true
});
