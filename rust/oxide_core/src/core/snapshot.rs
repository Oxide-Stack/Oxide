/// Versioned snapshot of store state.
///
/// A snapshot is broadcast to subscribers every time an action is dispatched.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "state-persistence",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct StateSnapshot<T> {
    /// Monotonically increasing revision number, starting at `0`.
    pub revision: u64,
    /// The state value for the given revision.
    pub state: T,
}
