pub mod bench_detail_route;
/// Application routes for the benchmark example.
///
/// This module is scanned by `#[routes]` to generate `RouteKind`, `RoutePayload`,
/// and the metadata file consumed by Dart code generation.
pub mod charts_route;
pub mod home_route;
pub mod routing_bench_route;
pub mod splash_route;

pub use bench_detail_route::BenchDetailRoute;
pub use charts_route::ChartsRoute;
pub use home_route::HomeRoute;
pub use routing_bench_route::RoutingBenchRoute;
pub use splash_route::SplashRoute;
