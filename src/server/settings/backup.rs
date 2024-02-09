crate::settings!(BackupSettings {
    /// Minimum=1 Maximum=300 Default=5
    #[serde(with = "crate::serde::u32")]
    backups_count: u32 = 5,

    /// Back up the world on server start
    #[serde(with = "crate::serde::bool")]
    backups_on_start: bool = true,

    /// Back up the world when versions change
    #[serde(with = "crate::serde::bool")]
    backups_on_version_change: bool = true,

    /// Minimum=0 Maximum=1500 Default=0
    #[serde(with = "crate::serde::u32")]
    backups_period: u32 = 0
});
