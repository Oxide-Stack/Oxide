use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::navigation::{DefaultExtra, RouteReturn};

/// A typed application route.
///
/// Route values are owned by Rust, serialized across the bridge, and executed by a Flutter
/// navigation handler (Navigator 1.0 or GoRouter).
///
/// # Deep links
///
/// A route can optionally describe how to map itself to and from a URL:
///
/// - [`Route::path`] defines an addressable template (e.g. `"/player/:trackId"`).
/// - [`Route::params`] provides template parameter substitutions.
/// - [`Route::query`] provides query parameters.
///
/// Extras returned by [`Route::extras`] are not encoded into the URL and are only available
/// when navigation is initiated within the running app.
///
/// # Example
///
/// ```rust
/// use oxide_core::navigation::{NoExtra, NoReturn, Route};
/// use serde::{Deserialize, Serialize};
/// use std::collections::HashMap;
///
/// #[derive(Clone, Serialize, Deserialize)]
/// pub struct PlayerRoute {
///     pub track_id: String,
///     pub autoplay: bool,
/// }
///
/// impl Route for PlayerRoute {
///     fn path() -> Option<&'static str> {
///         Some("/player/:trackId")
///     }
///
///     fn params(&self) -> HashMap<&'static str, String> {
///         HashMap::from([("trackId", self.track_id.clone())])
///     }
///
///     fn query(&self) -> HashMap<&'static str, String> {
///         HashMap::from([("autoplay", self.autoplay.to_string())])
///     }
///
///     type Return = NoReturn;
///     type Extra = NoExtra;
/// }
/// ```
pub trait Route: Clone + Serialize + DeserializeOwned + Send + Sync + 'static {
    /// Optional URL path template for deep links.
    fn path() -> Option<&'static str> {
        None
    }

    /// Path parameter mapping for deep link serialization.
    fn params(&self) -> HashMap<&'static str, String> {
        HashMap::new()
    }

    /// Query parameter mapping for deep link serialization.
    fn query(&self) -> HashMap<&'static str, String> {
        HashMap::new()
    }

    /// Optional extra payload passed out-of-band (not part of deep link URLs).
    fn extras(&self) -> Option<Self::Extra> {
        None
    }

    /// Return type of this route.
    ///
    /// If `Return = NoReturn`, the route is non-awaitable.
    type Return: RouteReturn;

    /// Optional extra type used by this route.
    type Extra: DefaultExtra;
}
