use oxide_generator_rs::state;

/// State for the ticker example.
#[flutter_rust_bridge::frb(non_opaque)]
#[state]
pub struct AppState {
    /// Number of ticks observed so far.
    pub ticks: u64,
    /// Whether auto ticking is currently enabled.
    pub is_running: bool,
    /// Tick interval used by the auto ticker (milliseconds).
    pub interval_ms: u64,
    /// Most recent tick source (`auto`, `manual`, or `side_effect`).
    pub last_tick_source: String,
}

impl AppState {
    /// Creates a new state instance.
    ///
    /// # Returns
    /// A state with `ticks = 0`.
    pub fn new() -> Self {
        Self {
            ticks: 0,
            is_running: false,
            interval_ms: 1000,
            last_tick_source: "manual".to_string(),
        }
    }
}

