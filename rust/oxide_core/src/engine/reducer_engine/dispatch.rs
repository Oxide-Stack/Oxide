use tokio::sync::watch;

use crate::engine::{Context, CoreResult, OxideError, Reducer, StateChange, StateSnapshot};

#[cfg(feature = "navigation-binding")]
use crate::engine::{NavigationCtx, navigation_runtime};

use super::ReducerEngine;

impl<R, StateSlice> ReducerEngine<R, StateSlice>
where
    R: Reducer<StateSlice>,
    StateSlice: Copy + PartialEq + Eq + Send + Sync + 'static,
{
    /// Dispatches an action and returns the resulting snapshot.
    ///
    /// If the reducer reports [`StateChange::None`], the current snapshot is returned and no
    /// new snapshot is broadcast.
    ///
    /// # Errors
    /// Returns any error produced by the reducer.
    pub async fn dispatch(
        &self,
        action: R::Action,
    ) -> CoreResult<StateSnapshot<R::State, StateSlice>> {
        tracing::debug!(target: "oxide::engine", "Dispatching action");
        let mut engine_state = self.shared.engine_state.lock().await;
        let before_snapshot = StateSnapshot {
            revision: engine_state.revision,
            state: engine_state.state.clone(),
            slices: Vec::new(),
        };

        #[cfg(feature = "navigation-binding")]
        let (runtime, route_ctx) = {
            let runtime = navigation_runtime()?;
            let route_ctx = runtime.current_route_context();
            (runtime, route_ctx)
        };

        // Apply reducer logic against a cloned state so failures never partially
        // mutate the committed state.
        let mut next_state = engine_state.state.clone();
        let ctx = Context {
            input: &action,
            state_snapshot: &before_snapshot,
            #[cfg(feature = "navigation-binding")]
            nav: NavigationCtx::new(runtime, &route_ctx),
        };
        let change = engine_state.reducer.reduce(&mut next_state, ctx)?;

        match change {
            // "no externally-visible change" should not spam watchers.
            StateChange::None => {
                tracing::trace!(target: "oxide::engine", "Action applied, no state change");
                Ok(before_snapshot)
            }
            StateChange::Full => {
                tracing::debug!(target: "oxide::engine", "Action applied, full state update");
                engine_state.state = next_state;
                engine_state.revision = engine_state.revision.saturating_add(1);

                let snapshot = StateSnapshot {
                    revision: engine_state.revision,
                    state: engine_state.state.clone(),
                    slices: Vec::new(),
                };
                let _ = self.shared.tx.send(snapshot.clone());
                self.persist_if_enabled(&snapshot);
                Ok(snapshot)
            }
            StateChange::Infer => {
                let slices = engine_state
                    .reducer
                    .infer_slices(&engine_state.state, &next_state);
                tracing::debug!(target: "oxide::engine", "Action applied, slice inferred update: {} slices", slices.len());

                engine_state.state = next_state;
                engine_state.revision = engine_state.revision.saturating_add(1);

                let snapshot = StateSnapshot {
                    revision: engine_state.revision,
                    state: engine_state.state.clone(),
                    slices,
                };
                let _ = self.shared.tx.send(snapshot.clone());
                self.persist_if_enabled(&snapshot);
                Ok(snapshot)
            }
            StateChange::Slices(slices) => {
                tracing::debug!(target: "oxide::engine", "Action applied, explicit slices matched: {} slices", slices.len());
                engine_state.state = next_state;
                engine_state.revision = engine_state.revision.saturating_add(1);

                let snapshot = StateSnapshot {
                    revision: engine_state.revision,
                    state: engine_state.state.clone(),
                    slices: slices.to_vec(),
                };
                let _ = self.shared.tx.send(snapshot.clone());
                self.persist_if_enabled(&snapshot);
                Ok(snapshot)
            }
        }
    }

    /// Returns the current snapshot without dispatching an action.
    pub async fn current(&self) -> StateSnapshot<R::State, StateSlice> {
        let engine_state = self.shared.engine_state.lock().await;
        StateSnapshot {
            revision: engine_state.revision,
            state: engine_state.state.clone(),
            slices: Vec::new(),
        }
    }

    /// Subscribes to snapshot updates.
    ///
    /// The returned receiver immediately contains the latest snapshot and will be notified on
    /// every subsequent committed update.
    pub fn subscribe(&self) -> watch::Receiver<StateSnapshot<R::State, StateSlice>> {
        self.shared.tx.subscribe()
    }

    /// Subscribes to engine errors that cannot be returned from the originating call site.
    ///
    /// This includes:
    /// - background side-effect failures (`Reducer::effect`)
    /// - persistence encoding failures (when `state-persistence` is enabled)
    /// - missing runtime dependencies (e.g., navigation runtime not initialized)
    pub fn subscribe_errors(&self) -> watch::Receiver<Option<OxideError>> {
        self.shared.error_tx.subscribe()
    }

    fn persist_if_enabled(&self, _snapshot: &StateSnapshot<R::State, StateSlice>) {
        #[cfg(feature = "state-persistence")]
        {
            if let Some(persistence) = &self.shared.persistence {
                match (persistence.encode)(&_snapshot.state) {
                    Ok(bytes) => {
                        if crate::persistence::debug_json_enabled() {
                            let debug_guard = persistence
                                .debug
                                .lock()
                                .unwrap_or_else(|e| e.into_inner());
                            if let Some(debug) = debug_guard.as_ref() {
                                match (debug.encode)(&_snapshot.state, &bytes) {
                                    Ok(json_bytes) => debug.worker.queue(json_bytes),
                                    Err(err) => {
                                        let _ = self.shared.error_tx.send(Some(err));
                                    }
                                }
                            }
                        }
                        persistence.worker.queue(bytes);
                    }
                    Err(err) => {
                        let _ = self.shared.error_tx.send(Some(err));
                    }
                }
            }
        }
    }
}
