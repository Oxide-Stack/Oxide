//! FFI-facing helpers.
//!
//! This module contains small utility types intended for consumption by other
//! runtimes and bindings layers.

mod error;
mod runtime;
mod stream;

pub use error::OxideError;
pub use runtime::init_tokio_runtime;
pub use runtime::{try_set_tokio_spawn_handle, try_set_tokio_spawn_runtime};
pub use stream::watch_receiver_to_stream;
