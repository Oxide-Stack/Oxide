use serde::Serialize;
use serde::de::DeserializeOwned;

/// Marker trait for route return types.
///
/// results are forwarded from Dart to Rust when a pushed route completes.
/// When a route does not return a value, its return type must be [`crate::navigation::NoReturn`].
pub trait RouteReturn: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T> RouteReturn for T where T: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {}
