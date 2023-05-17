crate::settings!(LatencySettings {
    /// Ping limit, in milliseconds, before a player is kicked from the server. (Set to 100 to disable)\nMinimum=100 Maximum=2147483647 Default=400
    #[serde(with = "crate::serde::u32")]
    ping_limit: u32 = 400,

    /// Number of days before old blood splats are removed.
    /// Removal happens when map chunks are loaded.
    /// Zero means they will never disappear\nMinimum=0 Maximum=365 Default=0
    #[serde(with = "crate::serde::u32")]
    blood_splat_lifespan_days: u32 = 0,

    /// If enabled, when HoursForCorpseRemoval triggers, it will also remove player�s corpses from the ground.
    #[serde(with = "crate::serde::bool")]
    remove_player_corpses_on_corpse_removal: bool = false,

    /// If true, player can use the "delete all" button on bins.
    #[serde(with = "crate::serde::bool")]
    trash_delete_all: bool = false,

    /// Modify the range of zombie attraction to cars. (Lower values can help with lag.)\nMinimum=0.00 Maximum=10.00 Default=0.50
    #[serde(with = "crate::serde::f32")]
    car_engine_attraction_modifier: f32 = 0.5
});
