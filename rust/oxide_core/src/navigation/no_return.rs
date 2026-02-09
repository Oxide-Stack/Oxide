use serde::{Deserialize, Serialize};

/// Marker return type indicating the route is not awaitable.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NoReturn;
