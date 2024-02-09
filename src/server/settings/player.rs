crate::settings!(PlayerSettings {
    /// If true, players will have to mouse over someone to see their display
    /// name.
    #[serde(with = "crate::serde::bool")]
    mouse_over_to_see_display_name: bool = true,

    /// Show Steam usernames and avatars in the Players list. Can be true
    /// (visible to everyone), false (visible to no one), or admin (visible to
    /// only admins)
    #[serde(with = "crate::serde::bool")]
    steam_scoreboard: bool = true,

    /// If true, automatically hide the player you can't see (like zombies).
    #[serde(with = "crate::serde::bool")]
    hide_players_behind_you: bool = true,

    /// Players can respawn in-game at the coordinates where they died
    #[serde(with = "crate::serde::bool")]
    player_respawn_with_self: bool = false,

    /// Players can respawn in-game at a split screen / Remote Play player's
    /// location
    #[serde(with = "crate::serde::bool")]
    player_respawn_with_other: bool = false,

    /// Governs whether players bump (and knock over) other players when
    /// running through them.
    #[serde(with = "crate::serde::bool")]
    player_bump_player: bool = false,

    /// Controls display of remote players on the in-game map.\n1=Hidden
    /// 2=Friends 3=Everyone\nMinimum=1 Maximum=3 Default=1
    #[serde(with = "crate::serde::u32")]
    map_remote_player_visibility: u32 = 1,

    ///
    #[serde(with = "crate::serde::bool")]
    sneak_mode_hide_from_other_players: bool = true
});
