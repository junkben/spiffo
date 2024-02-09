crate::settings!(ServerLogSettings {
    /// Semicolon-separated list of commands that will not be written to the
    /// cmd.txt server log. For example: \n-vehicle. Inputting * means do NOT
    /// write any vehicle command. Inputting: \n+vehicle.installPart means DO
    /// write that command
    client_command_filter: String = format!(
        "-vehicle.*;+vehicle.damageWindow;+vehicle.fixPart;+vehicle.installPart;+vehicle.\
         uninstallPart"
    ),

    /// Semicolon-separated list of actions that will be written to the
    /// ClientActionLogs.txt server log.
    client_action_logs: String = format!("ISEnterVehicle;ISExitVehicle;ISTakeEngineParts;"),

    /// Track changes in player perk levels in PerkLog.txt server log
    #[serde(with = "crate::serde::bool")]
    perk_logs: bool = true
});
