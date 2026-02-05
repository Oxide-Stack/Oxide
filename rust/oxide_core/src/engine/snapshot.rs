// Snapshot model used by both Rust and binding layers.
//
// Why: Exposing a small `{revision, state}` payload makes it easy for consumers
// to reason about update ordering and to build reactive streams on top.
/// Versioned snapshot of store state.
///
/// A snapshot is broadcast to subscribers when the engine commits a new state
/// (i.e. when a reducer returns [`crate::StateChange::FullUpdate`]).
///
/// Revisions start at `0` and increment by 1 for each committed update.
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
