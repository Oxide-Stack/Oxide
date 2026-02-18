use crate::navigation::RouteExtra;

/// A route extra that can provide a default fallback value.
///
/// How: deep links cannot provide extras (they are not part of the URL), so the runtime uses
/// `default_extra()` when no extras are available.
pub trait DefaultExtra: RouteExtra {
    /// Creates a default extra used when no extras are provided.
    fn default_extra() -> Self;
}
