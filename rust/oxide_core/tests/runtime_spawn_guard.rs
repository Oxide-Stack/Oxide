#![cfg(debug_assertions)]

#[test]
fn runtime_spawn_panics_when_uninitialized() {
    let result = std::panic::catch_unwind(|| {
        let _ = oxide_core::runtime::spawn(async move { 1u32 });
    });
    assert!(result.is_err());
}
