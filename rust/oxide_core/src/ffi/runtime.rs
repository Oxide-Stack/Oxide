/// Initializes Oxide's global Tokio runtime.
///
/// Oxide uses a Tokio runtime to drive reducer side-effects and background tasks.
/// In Flutter apps using `flutter_rust_bridge`, this function is intended to be
/// called from the FRB `init` entrypoint so a runtime exists before any engine
/// is created.
///
/// On non-wasm targets this creates (or reuses) a multi-thread Tokio runtime
/// with `enable_all()`. On wasm targets this is a no-op.
pub fn init_tokio_runtime() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        #[cfg(feature = "internal-runtime")]
        {
            crate::core::task::init_global_runtime();
        }
    }
}

/// Sets a global Tokio runtime handle used by Oxide for spawning background tasks.
///
/// This is an optional override. If not set, Oxide falls back to:
/// - the current Tokio runtime handle (when called from inside a runtime), then
/// - the Oxide internal runtime (when `internal-runtime` is enabled).
///
/// Returns `true` if the handle was set by this call. Returns `false` if a handle
/// was already set previously.
pub fn try_set_tokio_spawn_handle(handle: tokio::runtime::Handle) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return crate::core::task::try_set_global_spawn_handle(handle);
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = handle;
        false
    }
}

/// Sets a global Tokio runtime used by Oxide for spawning background tasks.
///
/// This stores the runtime's handle internally (the runtime itself is not owned by Oxide).
pub fn try_set_tokio_spawn_runtime(runtime: &tokio::runtime::Runtime) -> bool {
    try_set_tokio_spawn_handle(runtime.handle().clone())
}
