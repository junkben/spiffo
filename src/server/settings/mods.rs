crate::settings!(ModSettings {
    /// Enter the mod loading ID here. It can be found in
    /// \Steam\steamapps\workshop\modID\mods\modName\info.txt
    mods: String = format!(""),

    /// Enter the foldername of the mod found in
    /// \Steam\steamapps\workshop\modID\mods\modName\media\maps\
    map: String = format!("Muldraugh, KY"),

    /// List Workshop Mod IDs for the server to download. Each must be
    /// separated by a semicolon. Example:
    /// WorkshopItems=514427485;513111049
    workshop_items: String = format!("")
});
