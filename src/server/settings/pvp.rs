crate::settings!(PvpSettings {
    /// Players can hurt and kill other players
    #[serde(rename = "PVP")]
    #[serde(with = "crate::serde::bool")]
    pvp: bool = true,

    /// Players can enter and leave PVP on an individual basis. A player can
    /// only hurt another player when at least one of them is in PVP mode - as
    /// shown by the unobscured skull and crossbones on the left of the screen.
    /// When SafetySystem=false, players are free to hurt each other at any
    /// time if PVP is enabled.
    #[serde(with = "crate::serde::bool")]
    safety_system: bool = true,

    /// Display a skull icon over the head of players who have entered PVP mode
    #[serde(with = "crate::serde::bool")]
    show_safety: bool = true,

    /// The time it takes for a player to enter and leave PVP mode\nMinimum=0
    /// Maximum=1000 Default=2
    #[serde(with = "crate::serde::u32")]
    safety_toggle_timer: u32 = 2,

    /// The delay before a player can enter or leave PVP mode again, having
    /// recently done so\nMinimum=0 Maximum=1000 Default=3
    #[serde(with = "crate::serde::u32")]
    safety_cooldown_timer: u32 = 3,

    /// If true, player can hit again when struck by another player.
    #[serde(rename = "PVPMeleeWhileHitReaction")]
    #[serde(with = "crate::serde::bool")]
    pvp_melee_while_hit_reaction: bool = false,

    /// Damage multiplier for PVP melee attacks.\nMinimum=0.00 Maximum=500.00
    /// Default=30.00
    #[serde(rename = "PVPMeleeDamageModifier")]
    #[serde(with = "crate::serde::f32")]
    pvp_melee_damage_modifier: f32 = 30.0,

    /// Damage multiplier for PVP ranged attacks.\nMinimum=0.00 Maximum=500.00
    /// Default=50.00
    #[serde(with = "crate::serde::f32")]
    #[serde(rename = "PVPFirearmDamageModifier")]
    pvp_firearm_damage_modifier: f32 = 50.0
});
