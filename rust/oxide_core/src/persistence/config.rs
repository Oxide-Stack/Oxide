use std::time::Duration;

// Persistence configuration for throttling and keying.
//
// Why: Persistence needs stable addressing (key) and write debouncing (interval)
// to remain safe and efficient across platforms.
/// Configuration for persisting a store’s state to disk.
pub struct PersistenceConfig {
    /// A stable identifier used to derive the persistence file path.
    pub key: String,
    /// Minimum time between disk writes.
    ///
    /// Writes are debounced to avoid excessive disk churn when many actions are dispatched.
    pub min_interval: Duration,
}
