//! Ensures that logging macros compile away in release builds.
//!
//! This test is only enabled in release mode (when `debug_assertions` is off) to
//! validate that `tracing` macros such as `trace!` do not evaluate their
//! arguments, which is a key property of `tracing` when compiled with
//! `release_max_level_off`.

#![cfg(not(debug_assertions))]

use tracing::trace;

#[test]
fn trace_macro_does_not_evaluate_arguments_in_release() {
    static mut SIDE_EFFECT_EXECUTED: bool = false;

    fn side_effect() -> &'static str {
        unsafe {
            SIDE_EFFECT_EXECUTED = true;
        }
        "should_not_be_evaluated"
    }

    // In `release_max_level_off`, trace! is completely stripped and should not
    // evaluate its arguments.
    trace!("{}", side_effect());

    unsafe {
        assert!(
            !SIDE_EFFECT_EXECUTED,
            "trace! evaluated arguments in release"
        );
    }
}
