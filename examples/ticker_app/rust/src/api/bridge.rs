use oxide_generator_rs::reducer;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::time::Duration;

use crate::state::{AppAction, AppState};

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
    }

    fn reduce(
        &mut self,
        state: &mut Self::State,
        action: Self::Action,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match action {
            AppAction::StartTicker { interval_ms } => {
                let next_interval = interval_ms.max(1);
                if state.is_running && state.interval_ms == next_interval {
                    return Ok(oxide_core::StateChange::None);
                }
                state.is_running = true;
                state.interval_ms = next_interval;
                self.ensure_ticker_thread(next_interval);
                Ok(oxide_core::StateChange::FullUpdate)
            }
            AppAction::StopTicker => {
                if !state.is_running {
                    return Ok(oxide_core::StateChange::None);
                }
                state.is_running = false;
                self.stop_ticker_thread();
                Ok(oxide_core::StateChange::FullUpdate)
            }
            AppAction::AutoTick => {
                if !state.is_running {
                    return Ok(oxide_core::StateChange::None);
                }
                let next = state.ticks.saturating_add(1);
                if next == state.ticks {
                    return Ok(oxide_core::StateChange::None);
                }
                state.ticks = next;
                state.last_tick_source = "auto".to_string();
                Ok(oxide_core::StateChange::FullUpdate)
            }
            AppAction::ManualTick => {
                let next = state.ticks.saturating_add(1);
                if next == state.ticks {
                    return Ok(oxide_core::StateChange::None);
                }
                state.ticks = next;
                state.last_tick_source = "manual".to_string();
                Ok(oxide_core::StateChange::FullUpdate)
            }
            AppAction::EmitSideEffectTick => {
                if let Some(tx) = self.sideeffect_tx.as_ref() {
                    let _ = tx.send(AppSideEffect::TickFromSideEffect);
                }
                Ok(oxide_core::StateChange::None)
            }
            AppAction::Reset => {
                if state.ticks == 0 && state.last_tick_source == "manual" {
                    return Ok(oxide_core::StateChange::None);
                }
                state.ticks = 0;
                state.last_tick_source = "manual".to_string();
                Ok(oxide_core::StateChange::FullUpdate)
            }
        }
    }

    fn effect(
        &mut self,
        state: &mut Self::State,
        effect: Self::SideEffect,
    ) -> oxide_core::CoreResult<oxide_core::StateChange> {
        match effect {
            AppSideEffect::AutoTickFromThread => {
                if !state.is_running {
                    return Ok(oxide_core::StateChange::None);
                }
                let next = state.ticks.saturating_add(1);
                if next == state.ticks {
                    return Ok(oxide_core::StateChange::None);
                }
                state.ticks = next;
                state.last_tick_source = "auto".to_string();
                Ok(oxide_core::StateChange::FullUpdate)
            }
            AppSideEffect::TickFromSideEffect => {
                let next = state.ticks.saturating_add(1);
                if next == state.ticks {
                    return Ok(oxide_core::StateChange::None);
                }
                state.ticks = next;
                state.last_tick_source = "side_effect".to_string();
                Ok(oxide_core::StateChange::FullUpdate)
            }
        }
    }
}

#[flutter_rust_bridge::frb(ignore)]
enum AppSideEffect {
    AutoTickFromThread,
    TickFromSideEffect,
}

#[flutter_rust_bridge::frb(ignore)]
struct AppRootReducer {
    sideeffect_tx: Option<oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>>,
    ticker: Option<_TickerTask>,
}

impl Default for AppRootReducer {
    fn default() -> Self {
        Self {
            sideeffect_tx: None,
            ticker: None,
        }
    }
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
        if let Some(ticker) = self.ticker.take() {
            let _ = ticker.stop_tx.send(());
            if let Some(join) = ticker.join.lock().expect("ticker join mutex").take() {
                join.abort();
            }
        }
    }
}

struct _TickerTask {
    interval_ms: Arc<AtomicU64>,
    stop_tx: oxide_core::tokio::sync::oneshot::Sender<()>,
    join: StdMutex<Option<flutter_rust_bridge::JoinHandle<()>>>,
}

impl _TickerTask {
    fn start(
        sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>,
        interval_ms: u64,
    ) -> Self {
        let interval_ms = Arc::new(AtomicU64::new(interval_ms.max(1)));
        let interval_clone = Arc::clone(&interval_ms);
        let (stop_tx, mut stop_rx) = oxide_core::tokio::sync::oneshot::channel::<()>();

        let join = oxide_core::runtime::spawn(async move {
            loop {
                let ms = interval_clone.load(Ordering::Relaxed).max(1);

                #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
                {
                    oxide_core::tokio::select! {
                        _ = oxide_core::tokio::time::sleep(Duration::from_millis(ms)) => {
                            let _ = sideeffect_tx.send(AppSideEffect::AutoTickFromThread);
                        }
                        _ = &mut stop_rx => {
                            break;
                        }
                    }
                }

                #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
                {
                    use futures::FutureExt as _;

                    let stop_fut = (&mut stop_rx).fuse();
                    let sleep_fut = gloo_timers::future::TimeoutFuture::new(ms.min(u64::from(u32::MAX)) as u32).fuse();
                    futures::pin_mut!(stop_fut, sleep_fut);

                    futures::select_biased! {
                        _ = stop_fut => break,
                        _ = sleep_fut => {
                            let _ = sideeffect_tx.send(AppSideEffect::AutoTickFromThread);
                        }
                    }
                }
            }
        });

        Self {
            interval_ms,
            stop_tx,
            join: StdMutex::new(Some(join)),
        }
    }
}

#[flutter_rust_bridge::frb(init)]
/// Initializes Flutter Rust Bridge for this library.
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

#[flutter_rust_bridge::frb]
pub async fn init_oxide() -> Result<(), oxide_core::OxideError> {
    fn thread_pool() -> oxide_core::runtime::ThreadPool {
        crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
    }

    let _ = oxide_core::runtime::init(thread_pool);
    Ok(())
}
