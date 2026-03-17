//! Tests for Rust logging setup and correctness.

use tracing::trace;

#[test]
fn setup_log_stream_initializes_once() {
    // This test ensures setup_log_stream can be called multiple times without issues
    let rx1 = oxide_core::ffi::logger::setup_log_stream();
    let rx2 = oxide_core::ffi::logger::setup_log_stream();

    // First call should succeed, second should return None
    assert!(rx1.is_some());
    assert!(rx2.is_none());
}

#[test]
fn tracing_is_enabled_in_debug() {
    #[cfg(debug_assertions)]
    {
        // In debug, tracing should be enabled
        trace!("This should be captured if logging is set up");
        // We can't easily test the output without setting up the sink,
        // but we can ensure the macro compiles and doesn't panic
    }
}