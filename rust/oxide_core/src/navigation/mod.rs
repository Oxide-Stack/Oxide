//! Typed navigation primitives (Rust-driven, Flutter-native).
//!
//! This module is only available when the `navigation-binding` feature is enabled.
//!
//! # Design goals
//!
//! - Keep navigation logic separate from reducers and effects.
//! - Make routes type-safe in Rust while still interoperating with Flutter navigation.
//! - Provide a stable, minimal data model for bridging to Dart (via JSON payloads).
//!
//! # Usage
//!
//! Route types are typically defined in an application crate and registered via the
//! `#[routes]` macro from `oxide_generator_rs`. The generated `RouteKind` and
//! `RoutePayload` are then used by the navigation runtime to send commands to Dart
//! and to receive current-route updates.
//!
//! ```rust
//! use oxide_core::navigation::{NoExtra, NoReturn, Route};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Clone, Serialize, Deserialize)]
//! pub struct SplashRoute;
//!
//! impl Route for SplashRoute {
//!     type Return = NoReturn;
//!     type Extra = NoExtra;
//! }
//! ```
mod default_extra;
mod nav_command;
mod nav_route;
mod default_return;
mod no_extra;
mod no_return;
mod route;
mod route_context;
mod route_extra;
mod route_return;

pub use default_extra::DefaultExtra;
pub use default_return::DefaultReturn;
pub use nav_command::NavCommand;
pub use nav_route::NavRoute;
pub use no_extra::NoExtra;
pub use no_return::NoReturn;
pub use route::Route;
pub use route_context::RouteContext;
pub use route_extra::RouteExtra;
pub use route_return::RouteReturn;
