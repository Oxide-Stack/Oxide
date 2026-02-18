use serde::de::DeserializeOwned;
use serde::Serialize;

/// Marker trait for route extras.
///
/// Why: extras represent data that should not be encoded into a URL (deep link), but can still
/// be forwarded in-memory across the Rust ↔ Dart boundary.
pub trait RouteExtra: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T> RouteExtra for T where T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}
