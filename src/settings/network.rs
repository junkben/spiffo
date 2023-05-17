crate::settings!(NetworkSettings {
    /// Default starting port for player data. If UDP, this is this one of two ports used.\nMinimum=0 Maximum=65535 Default=16261
    #[serde(with = "crate::serde::u16")]
    default_port: u16 = 16261,

    /// Minimum=0 Maximum=65535 Default=16262
    #[serde(rename = "UDPPort")]
    #[serde(with = "crate::serde::u16")]
    udp_port: u16 = 16262,

    /// Reset ID determines if the server has undergone a soft-reset. If this number does match the client, the client must create a new character. Used in conjunction with PlayerServerID. It is strongly advised that you backup these IDs somewhere\nMinimum=0 Maximum=2147483647 Default=895534392
    #[serde(rename = "ResetID")]
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

    /// Clients must know this password to join the server. (Ignored when hosting a server via the Host button)
    password: String = format!(""),

    /// Attempt to configure a UPnP-enabled internet gateway to automatically setup port forwarding rules. The server will fall back to default ports if this fails
    #[serde(rename = "UPnP")]
    #[serde(with = "crate::serde::bool")]
    upnp: bool = true,

    #[serde(with = "crate::serde::bool")]
    login_queue_enabled: bool = false,

    /// Minimum=20 Maximum=1200 Default=60
    #[serde(with = "crate::serde::u32")]
    login_queue_connect_timeout: u32 = 60,

    /// Set the IP from which the server is broadcast. This is for network configurations with multiple IP addresses, such as server farms
    #[serde(rename = "server_browser_announced_ip")]
    server_browser_announced_ip: String = format!(""),

    /// The port for the RCON (Remote Console)\nMinimum=0 Maximum=65535 Default=27015
    #[serde(rename = "RCONPort")]
    #[serde(with = "crate::serde::u16")]
    rcon_port: u16 = 27015,

    /// RCON password (Pick a strong password)
    #[serde(rename = "RCONPassword")]
    rcon_password: String = format!("")
});
