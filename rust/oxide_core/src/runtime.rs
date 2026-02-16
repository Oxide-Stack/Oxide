//! Async runtime utilities backed by Flutter Rust Bridge (FRB).
//!
//! Oxide intentionally does not create or own its own executor. Instead, all
//! background work is spawned through FRB so it can run correctly on:
//!
//! - native platforms (threaded execution)
//! - WebAssembly in the browser (single-threaded `spawn_local`)
//! - WASI runtimes (executor-provided)
//!
//! ## Initialization requirement
//!
//! The FRB spawning APIs require access to a thread pool provider. Consumers
//! must initialize Oxide once at application startup (typically exposed as an
//! `initOxide()` function in their FRB API module) before creating any engines.
//!
//! In other words, call `runtime::init(...)` after `RustLib.init()` (Dart) and
//! before `ReducerEngine::new(...)`.

// Maintenance: keep this module free of store/reducer semantics. It should only
// provide spawn primitives and initialization checks used by other modules.
use std::future::Future;
use std::sync::OnceLock;

use crate::CoreResult;

#[cfg(feature = "frb-spawn")]
#[cfg(all(feature = "frb-spawn", target_arch = "wasm32"))]
/// Thread pool provider used by `spawn_blocking` on WebAssembly targets.
///
/// On web, FRB exposes a thread pool via a `thread_local!` key, so the runtime
/// stores a reference to that key rather than a direct pool reference.
pub type ThreadPool = &'static std::thread::LocalKey<flutter_rust_bridge::SimpleThreadPool>;

#[cfg(all(feature = "frb-spawn", not(target_arch = "wasm32")))]
/// Thread pool provider used by `spawn_blocking` on native targets.
pub type ThreadPool = &'static flutter_rust_bridge::SimpleThreadPool;

#[cfg(feature = "frb-spawn")]
type ThreadPoolProvider = fn() -> ThreadPool;

#[cfg(not(feature = "frb-spawn"))]
type ThreadPoolProvider = fn() -> ();

#[cfg(feature = "frb-spawn")]
static THREAD_POOL_PROVIDER: OnceLock<ThreadPoolProvider> = OnceLock::new();

#[cfg(feature = "frb-spawn")]
static INITIALIZED: OnceLock<()> = OnceLock::new();

#[cfg(feature = "frb-spawn")]
/// Initializes the Oxide runtime.
///
/// This should be called once during application startup. Subsequent calls are
/// harmless but return `false`.
///
/// Most applications expose this through their FRB API module as an `initOxide`
/// function so Dart can call it after `RustLib.init()`.
pub fn init(thread_pool_provider: ThreadPoolProvider) -> bool {
    let provider_set = THREAD_POOL_PROVIDER.set(thread_pool_provider).is_ok();
    let init_set = INITIALIZED.set(()).is_ok();
    provider_set && init_set
}

#[cfg(feature = "frb-spawn")]
/// Returns an error if the runtime has not been initialized.
pub fn ensure_initialized() -> CoreResult<()> {
    if INITIALIZED.get().is_some() && THREAD_POOL_PROVIDER.get().is_some() {
        return Ok(());
    }

    Err(crate::OxideError::Validation {
        message:
            "oxide_core runtime not initialized; call initOxide() from Dart main after RustLib.init()"
                .to_string(),
    })
}

#[cfg(feature = "frb-spawn")]
/// Returns the FRB thread pool needed for blocking work.
///
/// This is primarily intended for internal use and for reducers that need to
/// offload blocking work from the async task context.
pub fn thread_pool() -> CoreResult<ThreadPool> {
    ensure_initialized()?;
    let provider = THREAD_POOL_PROVIDER
        .get()
        .expect("thread pool provider present after ensure_initialized");
    Ok(provider())
}

#[cfg(feature = "frb-spawn")]
/// Spawns a background task via Flutter Rust Bridge.
///
/// Consumers generally should not call FRB spawning directly; Oxide uses this
/// to run side-effect processing and persistence workers.
pub fn spawn<F>(future: F) -> flutter_rust_bridge::JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    ensure_initialized().expect("oxide_core runtime must be initialized before spawning");
    flutter_rust_bridge::spawn(future)
}

#[cfg(all(feature = "frb-spawn", target_arch = "wasm32", target_os = "unknown"))]
pub fn spawn_local<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    ensure_initialized().expect("oxide_core runtime must be initialized before spawning");
    wasm_bindgen_futures::spawn_local(future);
}

#[cfg(all(feature = "frb-spawn", target_arch = "wasm32", target_os = "unknown"))]
/// Spawns a background task safely, using `spawn_local` on WASM.
pub fn safe_spawn<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    spawn_local(future);
}

#[cfg(all(feature = "frb-spawn", not(target_arch = "wasm32")))]
/// Spawns a background task safely, using `spawn` on native.
pub fn safe_spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    spawn(future);
}

#[cfg(all(feature = "frb-spawn", target_arch = "wasm32", target_os = "wasi"))]
/// Spawns a background task safely on WASI targets.
pub fn safe_spawn<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    spawn(future);
}

#[cfg(feature = "frb-spawn")]
/// Spawns a blocking function via Flutter Rust Bridge.
///
/// On native targets, this uses FRB's thread pool directly. On web targets,
/// FRB expects a reference to the thread-local thread pool key.
pub fn spawn_blocking<F, R>(f: F) -> flutter_rust_bridge::JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    ensure_initialized().expect("oxide_core runtime must be initialized before spawning");
    let pool = thread_pool().expect("thread pool present after ensure_initialized");
    #[cfg(target_arch = "wasm32")]
    {
        flutter_rust_bridge::spawn_blocking_with(f, &pool)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        flutter_rust_bridge::spawn_blocking_with(f, pool)
    }
}

#[cfg(not(feature = "frb-spawn"))]
pub fn init(_: ThreadPoolProvider) -> bool {
    false
}

#[cfg(not(feature = "frb-spawn"))]
pub fn ensure_initialized() -> CoreResult<()> {
    Err(crate::OxideError::Validation {
        message: "oxide_core built without frb-spawn; enable the `frb-spawn` feature".to_string(),
    })
}

#[cfg(not(feature = "frb-spawn"))]
pub fn spawn<F>(_future: F) -> ()
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    unreachable!("oxide_core built without frb-spawn; enable the `frb-spawn` feature")
}

#[cfg(not(feature = "frb-spawn"))]
pub fn safe_spawn<F>(_future: F)
where
    F: Future<Output = ()> + 'static,
{
    unreachable!("oxide_core built without frb-spawn; enable the `frb-spawn` feature")
}

#[cfg(not(feature = "frb-spawn"))]
pub fn spawn_blocking<F, R>(_f: F) -> ()
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    unreachable!("oxide_core built without frb-spawn; enable the `frb-spawn` feature")
}
