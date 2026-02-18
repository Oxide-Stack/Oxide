/// Application routes for the benchmark example.
///
/// This module is scanned by `#[routes]` to generate `RouteKind`, `RoutePayload`,
/// and the metadata file consumed by Dart code generation.
mod charts_route;
mod bench_detail_route;
mod home_route;
mod routing_bench_route;
mod splash_route;

pub use charts_route::ChartsRoute;
pub use bench_detail_route::BenchDetailRoute;
pub use home_route::HomeRoute;
pub use routing_bench_route::RoutingBenchRoute;
pub use splash_route::SplashRoute;
