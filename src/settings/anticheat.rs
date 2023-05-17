crate::settings!(AntiCheatSettings {
    /// Kick players that appear to be moving faster than is possible. May be buggy -- use with caution.
    #[serde(with = "crate::serde::bool")]
    kick_fast_players: bool = false,

    /// Enable the Steam VAC system
    #[serde(rename = "SteamVAC")]
    #[serde(with = "crate::serde::bool")]
    steam_vac: bool = true,

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
});
