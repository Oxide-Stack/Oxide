#[cfg(feature = "frb-spawn")]
#[test]
fn runtime_guard_and_init_paths() {
    let err = oxide_core::runtime::ensure_initialized().unwrap_err();
    assert!(err.to_string().contains("not initialized"));

    let pool_err = oxide_core::runtime::thread_pool().unwrap_err();
    assert!(pool_err.to_string().contains("not initialized"));

    fn thread_pool() -> &'static flutter_rust_bridge::SimpleThreadPool {
        static POOL: std::sync::OnceLock<flutter_rust_bridge::SimpleThreadPool> =
            std::sync::OnceLock::new();
        POOL.get_or_init(flutter_rust_bridge::SimpleThreadPool::default)
    }

    assert!(oxide_core::runtime::init(thread_pool));
    assert!(!oxide_core::runtime::init(thread_pool));
    oxide_core::runtime::ensure_initialized().unwrap();
    let _ = oxide_core::runtime::thread_pool().unwrap();
}

#[cfg(feature = "navigation-binding")]
#[test]
fn navigation_singleton_requires_init_then_is_available() {
    let err = match oxide_core::navigation_runtime() {
        Ok(_) => panic!("navigation runtime unexpectedly initialized"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("navigation runtime not initialized")
    );

    oxide_core::init_navigation().unwrap();
    let _runtime = oxide_core::navigation_runtime().unwrap();
}

#[cfg(feature = "isolated-channels")]
#[test]
fn isolated_channels_runtime_requires_init_then_is_available() {
    assert!(!oxide_core::isolated_channels_initialized());

    let err = match oxide_core::isolated_channels_runtime() {
        Ok(_) => panic!("isolated channels runtime unexpectedly initialized"),
        Err(err) => err,
    };
    assert!(
        err.to_string()
            .contains("isolated channels runtime not initialized")
    );
    assert_eq!(
        oxide_core::ensure_isolated_channels_initialized().unwrap_err(),
        oxide_core::OxideChannelError::Unavailable
    );

    oxide_core::init_isolated_channels().unwrap();
    assert!(oxide_core::isolated_channels_initialized());
    let _runtime = oxide_core::isolated_channels_runtime().unwrap();
    oxide_core::ensure_isolated_channels_initialized().unwrap();
}
