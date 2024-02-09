crate::settings!(VOIPSettings {
    /// VOIP is enabled when checked
    #[serde(with = "crate::serde::bool")]
    voice_enable: bool = true,

    /// The minimum tile distance over which VOIP sounds can be
    /// heard.\nMinimum=0.00 Maximum=100000.00 Default=10.00
    #[serde(with = "crate::serde::f32")]
    voice_min_distance: f32 = 10.0,

    /// The maximum tile distance over which VOIP sounds can be
    /// heard.\nMinimum=0.00 Maximum=100000.00 Default=100.00
    #[serde(with = "crate::serde::f32")]
    voice_max_distance: f32 = 100.0,

    /// Toggle directional audio for VOIP
    #[serde(rename = "Voice3D")]
    #[serde(with = "crate::serde::bool")]
    voice_3d: bool = true
});
