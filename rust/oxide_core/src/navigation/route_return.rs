use serde::de::DeserializeOwned;
use serde::Serialize;

/// Marker trait for route return types.
///
/// Why: results are forwarded from Dart to Rust when a pushed route completes.
/// When a route does not return a value, its return type must be [`crate::navigation::NoReturn`].
pub trait RouteReturn: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T> RouteReturn for T where T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}
