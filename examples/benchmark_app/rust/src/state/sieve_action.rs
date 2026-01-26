use oxide_macros::actions;

/// Actions for the sieve workload.
#[actions]
pub enum SieveAction {
    /// Run the sieve `iterations` times.
    Run { iterations: u32 },
}

