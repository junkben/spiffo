crate::settings!(FactionSettings {
    /// Players can create factions when true
    #[serde(with = "crate::serde::bool")]
    faction: bool = true,

    /// Players must survive this number of in-game days before being allowed to create a faction\nMinimum=0 Maximum=2147483647 Default=0
    #[serde(with = "crate::serde::u32")]
    faction_day_survived_to_create: u32 = 0,

    /// Number of players required as faction members before the faction owner can create a group tag\nMinimum=1 Maximum=2147483647 Default=1
    #[serde(with = "crate::serde::u32")]
    faction_players_required_for_tag: u32 = 1
});
