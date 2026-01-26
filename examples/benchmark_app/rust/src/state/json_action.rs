use oxide_macros::actions;

/// Actions for the JSON workload.
#[actions]
pub enum JsonAction {
    /// Decode + canonicalize + encode `assets/light.json` `iterations` times.
    RunLight { iterations: u32 },
    /// Decode + canonicalize + encode `assets/heavy.json` `iterations` times.
    RunHeavy { iterations: u32 },
}

