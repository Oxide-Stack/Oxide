use oxide_generator_rs::actions;

/// Actions for the sieve workload.
#[actions]
pub enum SieveAction {
    /// Run the sieve `iterations` times.
    Run { iterations: u32 },
}

