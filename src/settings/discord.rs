crate::settings!(DiscordSettings {
    /// Enables global text chat integration with a Discord channel
    #[serde(with = "crate::serde::bool")]
    discord_enable: bool = false,

    /// Discord bot access token
    discord_token: String = format!(""),

    /// The Discord channel name. (Try the separate channel ID option if having difficulties)
    discord_channel: String = format!(""),

    /// The Discord channel ID. (Use if having difficulties with Discord channel name option)
    #[serde(rename = "DiscordChannelID")]
    discord_channel_id: String = format!("")
});
