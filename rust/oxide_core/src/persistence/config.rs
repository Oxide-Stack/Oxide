use std::time::Duration;

/// Configuration for persisting a store’s state to disk.
pub struct PersistenceConfig {
    /// A stable identifier used to derive the persistence file path.
    pub key: String,
    /// Minimum time between disk writes.
    ///
    /// Writes are debounced to avoid excessive disk churn when many actions are dispatched.
    pub min_interval: Duration,
}
