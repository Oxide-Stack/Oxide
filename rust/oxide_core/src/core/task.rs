use std::future::Future;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::OnceLock;

#[cfg(all(not(target_arch = "wasm32"), feature = "internal-runtime"))]
static GLOBAL_RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
static GLOBAL_SPAWN_HANDLE: OnceLock<tokio::runtime::Handle> = OnceLock::new();

#[cfg(all(not(target_arch = "wasm32"), feature = "internal-runtime"))]
pub(crate) fn init_global_runtime() {
    let _ = global_runtime();
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn try_set_global_spawn_handle(handle: tokio::runtime::Handle) -> bool {
    GLOBAL_SPAWN_HANDLE.set(handle).is_ok()
}

pub(crate) fn spawn_detached<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    spawn_detached_with_handle::<F>(None, future);
}

pub(crate) fn spawn_detached_with_handle<F>(handle: Option<&tokio::runtime::Handle>, future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    // Spawn priority:
    // 1) explicit handle provided by caller
    // 2) current Tokio runtime handle (if we're already inside a runtime)
    // 3) user-provided global spawn handle
    // 4) Oxide internal global runtime fallback (feature-gated)
    if let Some(handle) = handle {
        handle.spawn(future);
        return;
    }

    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.spawn(future);
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    if let Some(handle) = GLOBAL_SPAWN_HANDLE.get() {
        handle.spawn(future);
        return;
    }

    #[cfg(target_arch = "wasm32")]
    {
        #[cfg(feature = "frb-spawn")]
        {
            let _ = flutter_rust_bridge::spawn(future);
        }

        #[cfg(not(feature = "frb-spawn"))]
        {
            panic!(
                "spawn_detached called without a Tokio runtime; enable the `frb-spawn` feature or create a Tokio runtime first"
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        #[cfg(feature = "internal-runtime")]
        {
            global_runtime().spawn(future);
            return;
        }

        #[cfg(not(feature = "internal-runtime"))]
        {
            panic!(
                "spawn_detached called without a Tokio runtime; provide a Tokio runtime/handle, or enable oxide_core's `internal-runtime` feature"
            );
        }
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "internal-runtime"))]
fn global_runtime() -> &'static tokio::runtime::Runtime {
    GLOBAL_RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("failed to build global Tokio runtime")
    })
}
