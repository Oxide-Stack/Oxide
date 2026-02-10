use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
///
/// # Examples
/// ```
/// use rust_lib_ticker_app::api::bridge::OxideError;
///
/// let _ = OxideError::Validation {
///     message: "example".to_string(),
/// };
/// ```
pub use oxide_core::OxideError;

use oxide_core::StateChange;
use oxide_core::tokio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::time::Duration;

use crate::state::{AppAction, AppState, AppStateSlice};
use crate::state::app_state::TickState;

#[reducer(
    engine = AppEngine,
    snapshot = AppStateSnapshot,
    initial = AppState::new(),
)]
impl oxide_core::Reducer for AppRootReducer {
    type State = AppState;
    type Action = AppAction;
    type SideEffect = AppSideEffect;

    async fn init(&mut self, ctx: oxide_core::InitContext<Self::SideEffect>) {
        self.sideeffect_tx = Some(ctx.sideeffect_tx);
        if let Ok(runtime) = oxide_core::navigation_runtime() {
            runtime.push(crate::routes::HomeRoute {});
        }
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::Action, Self::State, AppStateSlice>,
    ) -> oxide_core::CoreResult<StateChange<AppStateSlice>> {
        match ctx.input {
            AppAction::StartTicker { interval_ms } => {
                let next_interval = (*interval_ms).max(1);
                if state.control.is_running && state.control.interval_ms == next_interval {
                    return Ok(StateChange::None);
                }

                state.control.is_running = true;
                state.control.interval_ms = next_interval;
                self.ensure_ticker_thread(next_interval);
                Ok(StateChange::Infer)
            }
            AppAction::StopTicker => {
                if !state.control.is_running {
                    return Ok(StateChange::None);
                }

                state.control.is_running = false;
                self.stop_ticker_thread();
                Ok(StateChange::Infer)
            }
            AppAction::AutoTick => {
                if !state.control.is_running {
                    return Ok(StateChange::None);
                }

                if !increment_tick(&mut state.tick, "auto") {
                    return Ok(StateChange::None);
                }

                Ok(StateChange::Infer)
            }
            AppAction::ManualTick => {
                if !increment_tick(&mut state.tick, "manual") {
                    return Ok(StateChange::None);
                }

                Ok(StateChange::Infer)
            }
            AppAction::EmitSideEffectTick => {
                if let Some(tx) = self.sideeffect_tx.as_ref() {
                    let _ = tx.send(AppSideEffect::TickFromSideEffect);
                }
                Ok(StateChange::None)
            }
            AppAction::Reset => {
                if state.tick.ticks == 0 && state.tick.last_tick_source == "manual" {
                    return Ok(StateChange::None);
                }

                state.tick.ticks = 0;
                state.tick.last_tick_source = "manual".to_string();
                Ok(StateChange::Infer)
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        ctx: oxide_core::Context<'_, Self::SideEffect, Self::State, AppStateSlice>,
    ) -> oxide_core::CoreResult<StateChange<AppStateSlice>> {
        match ctx.input {
            AppSideEffect::AutoTickFromThread => {
                if !state.control.is_running {
                    return Ok(StateChange::None);
                }

                if !increment_tick(&mut state.tick, "auto") {
                    return Ok(StateChange::None);
                }

                Ok(StateChange::Infer)
            }
            AppSideEffect::TickFromSideEffect => {
                if !increment_tick(&mut state.tick, "side_effect") {
                    return Ok(StateChange::None);
                }

                Ok(StateChange::Infer)
            }
        }
    }
}

#[flutter_rust_bridge::frb(ignore)]
pub(crate) enum AppSideEffect {
    AutoTickFromThread,
    TickFromSideEffect,
}

#[flutter_rust_bridge::frb(ignore)]
#[derive(Default)]
pub(crate) struct AppRootReducer {
    sideeffect_tx: Option<tokio::sync::mpsc::UnboundedSender<AppSideEffect>>,
    ticker: Option<_TickerTask>,
}

impl AppRootReducer {
    fn ensure_ticker_thread(&mut self, interval_ms: u64) {
        if let Some(ticker) = self.ticker.as_ref() {
            ticker.interval_ms.store(interval_ms, Ordering::Relaxed);
            return;
        }

        let Some(tx) = self.sideeffect_tx.as_ref() else {
            return;
        };

        self.ticker = Some(_TickerTask::start(tx.clone(), interval_ms));
    }

    fn stop_ticker_thread(&mut self) {
        let Some(ticker) = self.ticker.take() else {
            return;
        };

        let _ = ticker.stop_tx.send(());
        let _ = ticker.join.lock().expect("ticker join mutex").take();
    }
}

fn increment_tick(tick: &mut TickState, source: &str) -> bool {
    let next = tick.ticks.saturating_add(1);
    if next == tick.ticks {
        return false;
    }
    tick.ticks = next;
    tick.last_tick_source = source.to_string();
    true
}

struct _TickerTask {
    interval_ms: Arc<AtomicU64>,
    stop_tx: tokio::sync::oneshot::Sender<()>,
    join: StdMutex<Option<flutter_rust_bridge::JoinHandle<()>>>,
}

impl _TickerTask {
    fn start(sideeffect_tx: tokio::sync::mpsc::UnboundedSender<AppSideEffect>, interval_ms: u64) -> Self {
        let interval_ms = Arc::new(AtomicU64::new(interval_ms.max(1)));
        let interval_clone = Arc::clone(&interval_ms);
        let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel::<()>();

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        {
            oxide_core::runtime::safe_spawn(async move {
                loop {
                    let ms = interval_clone.load(Ordering::Relaxed).max(1);

                    use futures::FutureExt as _;

                    let stop_fut = (&mut stop_rx).fuse();
                    let sleep_fut =
                        gloo_timers::future::TimeoutFuture::new(ms.min(u64::from(u32::MAX)) as u32).fuse();
                    futures::pin_mut!(stop_fut, sleep_fut);

                    futures::select_biased! {
                        _ = stop_fut => break,
                        _ = sleep_fut => {
                            let _ = sideeffect_tx.send(AppSideEffect::AutoTickFromThread);
                        }
                    }
                }
            });

            return Self {
                interval_ms,
                stop_tx,
                join: StdMutex::new(None),
            };
        }

        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        {
            let join = oxide_core::runtime::spawn(async move {
                loop {
                    let ms = interval_clone.load(Ordering::Relaxed).max(1);
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_millis(ms)) => {
                            let _ = sideeffect_tx.send(AppSideEffect::AutoTickFromThread);
                        }
                        _ = &mut stop_rx => {
                            break;
                        }
                    }
                }
            });

            return Self {
                interval_ms,
                stop_tx,
                join: StdMutex::new(Some(join)),
            };
        }
    }
}

#[flutter_rust_bridge::frb(init)]
/// Initializes Flutter Rust Bridge for this library.
///
/// # Examples
/// ```
/// use rust_lib_ticker_app::api::bridge::init_app;
///
/// init_app();
/// ```
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb]
/// Initializes the Oxide runtime for this example.
///
/// Call this once during app startup (after `RustLib.init()` on the Dart side)
/// before creating any engines.
///
/// # Examples
/// ```
/// use rust_lib_ticker_app::api::bridge::init_oxide;
///
/// tokio::runtime::Runtime::new()
///     .unwrap()
///     .block_on(async { init_oxide().await.unwrap() });
/// ```
pub async fn init_oxide() -> Result<(), oxide_core::OxideError> {
    fn thread_pool() -> oxide_core::runtime::ThreadPool {
        crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
    }

    let _ = oxide_core::runtime::init(thread_pool);
    crate::navigation::runtime::init()?;

    Ok(())
}

#[flutter_rust_bridge::frb]
pub async fn create_shared_engine() -> Result<Arc<AppEngine>, oxide_core::OxideError> {
    static ENGINE: tokio::sync::OnceCell<Arc<AppEngine>> = tokio::sync::OnceCell::const_new();
    let engine = ENGINE.get_or_try_init(|| async { create_engine().await }).await?;
    Ok(Arc::clone(engine))
}
