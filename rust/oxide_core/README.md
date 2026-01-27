# oxide_core

`oxide_core` is the Rust-side engine for Oxide. It provides the primitives for:

- Defining reducers (`Reducer`)
- Owning state in an engine (`ReducerEngine`)
- Streaming revisioned snapshots (`StateSnapshot<T>`)
- Optional persistence helpers (feature-gated)

This crate is intentionally usage-agnostic. For end-to-end Rust ↔ Flutter wiring, see the repository [examples](../../examples) and the root [README](../../README.md).

## Add It To Your Crate

In your `Cargo.toml`:

```toml
[dependencies]
oxide_core = "0.1.0"
```

When working inside this repository, use a combined version + path dependency (Cargo prefers `path` locally, while published crates resolve by `version`):

```toml
oxide_core = { version = "0.1.0", path = "../rust/oxide_core" }
```

## Core Concepts

### Reducer

Reducers are stateful controllers that mutate state in response to actions and side-effects:

```rust
use oxide_core::{CoreResult, Reducer, StateChange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CounterState {
  pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CounterAction {
  Inc,
}

pub enum CounterSideEffect {}

#[derive(Default)]
pub struct CounterReducer;
impl Reducer for CounterReducer {
  type State = CounterState;
  type Action = CounterAction;
  type SideEffect = CounterSideEffect;

  fn init(
    &mut self,
    _sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
  ) {}

  fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange> {
    match action {
      CounterAction::Inc => state.value = state.value.saturating_add(1),
    }
    Ok(StateChange::FullUpdate)
  }

  fn effect(
    &mut self,
    _state: &mut Self::State,
    _effect: Self::SideEffect,
  ) -> CoreResult<StateChange> {
    Ok(StateChange::None)
  }
}
```

### ReducerEngine

`ReducerEngine<R>` is the async-safe facade for dispatching actions and observing snapshots:

```rust
use oxide_core::ReducerEngine;

# use oxide_core::{CoreResult, Reducer, StateChange};
# use oxide_core::tokio::sync::mpsc;
# #[derive(Debug, Clone, PartialEq, Eq)]
# pub struct CounterState {
#   pub value: u64,
# }
# #[derive(Debug, Clone, PartialEq, Eq)]
# pub enum CounterAction {
#   Inc,
# }
# pub enum CounterSideEffect {}
# #[derive(Default)]
# pub struct CounterReducer;
# impl Reducer for CounterReducer {
#   type State = CounterState;
#   type Action = CounterAction;
#   type SideEffect = CounterSideEffect;
#
#   fn init(&mut self, _sideeffect_tx: mpsc::UnboundedSender<Self::SideEffect>) {}
#
#   fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> CoreResult<StateChange> {
#     match action {
#       CounterAction::Inc => state.value = state.value.saturating_add(1),
#     }
#     Ok(StateChange::FullUpdate)
#   }
#
#   fn effect(
#     &mut self,
#     _state: &mut Self::State,
#     _effect: Self::SideEffect,
#   ) -> CoreResult<StateChange> {
#     Ok(StateChange::None)
#   }
# }
let runtime = tokio::runtime::Runtime::new().unwrap();
runtime.block_on(async {
  let engine = ReducerEngine::<CounterReducer>::new(
    CounterReducer::default(),
    CounterState { value: 0 },
  );
  let snap = engine.dispatch(CounterAction::Inc).await.unwrap();
  let current = engine.current().await;
  assert_eq!(snap.revision, current.revision);
});
```

### Async Runtime Behavior

`ReducerEngine` starts an internal async loop to process side-effects. Engine creation is safe even when there is no ambient Tokio runtime (for example, when entered from a synchronous FFI boundary):

- If a Tokio runtime is currently available, Oxide spawns tasks onto it.
- Otherwise, when built with the default `frb-spawn` feature, Oxide uses Flutter Rust Bridge’s cross-platform `spawn` helper.
- If `frb-spawn` is disabled, Oxide falls back to an internal global Tokio runtime for background work.

In a normal Rust binary, the simplest approach is to run inside a Tokio runtime:

```rust,ignore
use oxide_core::ReducerEngine;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
  let engine = ReducerEngine::<CounterReducer>::new(CounterReducer::default(), CounterState { value: 0 });
  let _ = engine.dispatch(CounterAction::Inc).await.unwrap();
}
```

### Invariant: No Partial Mutation On Error

Dispatch uses clone-first semantics:

- The current state is cloned.
- The reducer runs against the clone.
- If the reducer returns `StateChange::None`, the clone is discarded and no snapshot is emitted.
- If the reducer returns an error, the live state is not updated and no snapshot is emitted.

This makes it safe to write reducers as normal `&mut State` code while preserving strong error semantics.

## Feature Flags

- `frb-spawn` (default): enables FRB’s `spawn` helper for cross-platform task spawning
- `state-persistence`: enables bincode encode/decode helpers in `oxide_core::persistence`
- `persistence-json`: adds JSON encode/decode helpers (requires `state-persistence`)
- `full`: enables all persistence features

## Commands

From `rust/`:

```bash
cargo test
```

To run benches (if enabled for your environment):

```bash
cargo bench
```

## License

MIT. See [LICENSE](../../LICENSE).
