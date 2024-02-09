crate::settings!(WorldSettings {
    /// Game time stops when there are no players online
    #[serde(with = "crate::serde::bool")]
    pause_empty: bool = true,

    /// After X hours, all containers in the world will respawn loot. To spawn
    /// loot a container must have been looted at least once. Loot respawn is
    /// not impacted by visibility or subsequent looting.\nMinimum=0
    /// Maximum=2147483647 Default=0
    #[serde(with = "crate::serde::u32")]
    hours_for_loot_respawn: u32 = 0,

    /// Containers with a number of items greater, or equal to, this setting
    /// will not respawn\nMinimum=1 Maximum=2147483647 Default=4
    #[serde(with = "crate::serde::u32")]
    max_items_for_loot_respawn: u32 = 4,

    /// Items will not respawn in buildings that players have barricaded or
    /// built in
    #[serde(with = "crate::serde::bool")]
    construction_prevents_loot_respawn: bool = true,

    /// All forms of fire are disabled - except for campfires
    #[serde(with = "crate::serde::bool")]
    no_fire: bool = false,

    /// The number of in-game minutes it takes to read one page of a
    /// book\nMinimum=0.00 Maximum=60.00 Default=1.00
    #[serde(with = "crate::serde::f32")]
    minutes_per_page: f32 = 1.0,

    /// Loaded parts of the map are saved after this set number of real-world
    /// minutes have passed. (The map is usually saved only after clients leave
    /// a loaded area)\nMinimum=0 Maximum=2147483647 Default=0
    #[serde(with = "crate::serde::u32")]
    save_world_every_minutes: u32 = 0,

    /// Allow players to destroy world objects with sledgehammers
    #[serde(with = "crate::serde::bool")]
    allow_destruction_by_sledgehammer: bool = true,

    /// Allow players to destroy world objects only in their safehouse (require
    /// AllowDestructionBySledgehammer to true).
    #[serde(with = "crate::serde::bool")]
    sledgehammer_only_in_safehouse: bool = false,

    /// Players are allowed to sleep when their survivor becomes tired, but
    /// they do not NEED to sleep
    #[serde(with = "crate::serde::bool")]
    sleep_allowed: bool = false,

    /// Players get tired and need to sleep. (Ignored if SleepAllowed=false)
    #[serde(with = "crate::serde::bool")]
    sleep_needed: bool = false,

    ///
    #[serde(with = "crate::serde::bool")]
    knocked_down_allowed: bool = true,

    /// Minimum=10.00 Maximum=150.00 Default=70.00
    #[serde(with = "crate::serde::f32")]
    speed_limit: f32 = 70.0,

    /// Governs how fast time passes while players sleep. Value multiplies the
    /// speed of the time that passes during sleeping.\nMinimum=1.00
    /// Maximum=100.00 Default=40.00
    #[serde(with = "crate::serde::f32")]
    fast_forward_multiplier: f32 = 40.0,

    /// Maximum number of items that can be placed in a container.  Zero means
    /// there is no limit. (PLEASE NOTE: This includes individual small items
    /// such as nails. A limit of 50 will mean only 50 nails can be
    /// stored.)\nMinimum=0 Maximum=9000 Default=0
    #[serde(with = "crate::serde::u32")]
    item_numbers_limit_per_container: u32 = 0
});
