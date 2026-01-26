use oxide_macros::actions;

/// Actions for the counter workload.
#[actions]
pub enum CounterAction {
    /// Perform `iterations` counter ticks.
    Run { iterations: u32 },
}

