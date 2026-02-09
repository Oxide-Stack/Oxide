/// Application routes for the benchmark example.
///
/// This module is scanned by `#[routes]` to generate `RouteKind`, `RoutePayload`,
/// and the metadata file consumed by Dart code generation.
mod charts_route;
mod home_route;
mod splash_route;

pub use charts_route::ChartsRoute;
pub use home_route::HomeRoute;
pub use splash_route::SplashRoute;
