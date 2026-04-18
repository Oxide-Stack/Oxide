mod args;
mod codegen;
mod discovery;
mod expand;
mod metadata;
mod model;
mod route_struct;

pub use args::OxideRouteArgs;
pub use expand::expand_routes_module;
pub use route_struct::expand_oxide_route_struct;

#[cfg(test)]
mod tests;
