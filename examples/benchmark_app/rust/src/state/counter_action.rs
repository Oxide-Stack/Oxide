use oxide_generator_rs::actions;

/// Actions for the counter workload.
#[actions]
pub enum CounterAction {
    /// Perform `iterations` counter ticks.
    Run { iterations: u32 },
}

