crate::settings!(TransmissionSettings {
    /// Disables radio transmissions from players with an access level
    #[serde(with = "crate::serde::bool")]
    disable_radio_staff: bool = false,

    /// Disables radio transmissions from players with 'admin' access level
    #[serde(with = "crate::serde::bool")]
    disable_radio_admin: bool = true,

    /// Disables radio transmissions from players with 'gm' access level
    #[serde(rename = "DisableRadioGM")]
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
});
