crate::settings!(SafehouseSettings {
    /// Both admins and players can claim safehouses
    #[serde(with = "crate::serde::bool")]
    player_safehouse: bool = false,

    /// Only admins can claim safehouses
    #[serde(with = "crate::serde::bool")]
    admin_safehouse: bool = false,

    /// Allow non-members to enter a safehouse without being invited
    #[serde(rename = "SafehouseAllowTrepass")]
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
    #[serde(rename = "SafeHouseRemovalTime")]
    #[serde(with = "crate::serde::u32")]
    safehouse_removal_time: u32 = 144,

    /// Governs whether players can claim non-residential buildings.
    #[serde(with = "crate::serde::bool")]
    safehouse_allow_non_residential: bool = false,

    /// Safehouse acts like a normal house if a member of the safehouse is connected (so secure when players are offline)
    #[serde(with = "crate::serde::bool")]
    disable_safehouse_when_player_connected: bool = false
});
