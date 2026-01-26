use oxide_macros::reducer;

/// Error type exposed across the FFI boundary.
pub use oxide_core::OxideError;

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::state::{AppAction, AppState};

#[reducer(
    engine = AppEngine,
    snapshot = AppStateSnapshot,
    initial = AppState::new(),
    tokio_handle = crate::runtime::handle()
)]
impl oxide_core::Reducer for AppReducer {
    type State = AppState;
    type Action = AppAction;
    type SideEffect = AppSideEffect;

    fn init(
        &mut self,
        sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
    ) {
        self.sideeffect_tx = Some(sideeffect_tx);
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

pub enum AppSideEffect {
    AutoTickFromThread,
    TickFromSideEffect,
}

#[flutter_rust_bridge::frb(opaque)]
pub struct AppReducer {
    sideeffect_tx: Option<oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>>,
    ticker: Option<_TickerTask>,
}

impl Default for AppReducer {
    fn default() -> Self {
        Self {
            sideeffect_tx: None,
            ticker: None,
        }
    }
}

impl AppReducer {
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
            ticker.join.abort();
        }
    }
}

struct _TickerTask {
    interval_ms: Arc<AtomicU64>,
    stop_tx: oxide_core::tokio::sync::oneshot::Sender<()>,
    join: oxide_core::tokio::task::JoinHandle<()>,
}

impl _TickerTask {
    fn start(
        sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<AppSideEffect>,
        interval_ms: u64,
    ) -> Self {
        let interval_ms = Arc::new(AtomicU64::new(interval_ms.max(1)));
        let interval_clone = Arc::clone(&interval_ms);
        let (stop_tx, mut stop_rx) = oxide_core::tokio::sync::oneshot::channel::<()>();

        let join = crate::runtime::handle().spawn(async move {
            loop {
                let ms = interval_clone.load(Ordering::Relaxed).max(1);
                oxide_core::tokio::select! {
                    _ = oxide_core::tokio::time::sleep(Duration::from_millis(ms)) => {
                        let _ = sideeffect_tx.send(AppSideEffect::AutoTickFromThread);
                    }
                    _ = &mut stop_rx => {
                        break;
                    }
                }
            }
        });

        Self {
            interval_ms,
            stop_tx,
            join,
        }
    }
}

#[flutter_rust_bridge::frb(init)]
/// Initializes Flutter Rust Bridge for this library.
pub fn init_app() {
    let _ = crate::runtime::runtime();
    flutter_rust_bridge::setup_default_user_utils();
}
