# Reducer Pattern (Rust)

This page covers the Rust-side pieces you define for an Oxide store:

- State (your data model)
- Actions (events from UI / system)
- Reducer (the pure-ish state transition function + side-effect entrypoint)

## Add Rust Dependencies

In your Rust crate (the one FRB will bind to), add dependencies:

```toml
[dependencies]
oxide_core = "0.3.0"
oxide_generator_rs = "0.3.0"
```

When working inside this repository, use combined version + path dependencies (Cargo prefers `path` locally, while published crates resolve by `version`):

```toml
oxide_core = { version = "0.3.0", path = "../rust/oxide_core" }
oxide_generator_rs = { version = "0.3.0", path = "../rust/oxide_generator_rs" }
```

## Define State, Actions, Reducer

This is the minimal pattern (modeled after `counter_app`).

```rust
use oxide_generator_rs::{actions, reducer, state};

#[state]
pub struct AppState {
  pub counter: u64,
}

#[actions]
pub enum AppAction {
  Increment,
}

pub enum AppSideEffect {}

#[derive(Default)]
pub struct AppReducer {}

#[reducer(
  engine = AppEngine,
  snapshot = AppStateSnapshot,
  initial = AppState { counter: 0 },
)]
impl oxide_core::Reducer for AppReducer {
  type State = AppState;
  type Action = AppAction;
  type SideEffect = AppSideEffect;

  async fn init(&mut self, _ctx: oxide_core::InitContext<Self::SideEffect>) {}

  fn reduce(
    &mut self,
    state: &mut Self::State,
    action: Self::Action,
  ) -> oxide_core::CoreResult<oxide_core::StateChange> {
    match action {
      AppAction::Increment => state.counter = state.counter.saturating_add(1),
    }
    Ok(oxide_core::StateChange::Full)
  }

  fn effect(
    &mut self,
    _state: &mut Self::State,
    _effect: Self::SideEffect,
  ) -> oxide_core::CoreResult<oxide_core::StateChange> {
    Ok(oxide_core::StateChange::None)
  }
}
```

## What The Macros Generate

At a high level, the macros generate:

- An engine type (held behind an `Arc` on the Dart side).
- A snapshot type for `current()` and stream updates.
- An FRB-friendly surface by default (can be disabled with `no_frb`).

## Emitting Updates: `StateChange`

Your reducer returns a `StateChange` that controls snapshot emissions:

- `StateChange::None`: don’t commit and don’t emit a snapshot.
- `StateChange::Full`: commit and emit a “full update”.

If you enable sliced updates (next section), you’ll also use:

- `StateChange::Infer`: compare top-level fields and infer which slices changed.
- `StateChange::Slices(&[...])`: explicitly declare which slices changed.

## Optional: Sliced Updates

Sliced updates let Flutter subscribe to only the parts of state it cares about (based on top-level state fields).

Rust side:

```rust
use oxide_generator_rs::{actions, reducer, state};

#[state(sliced = true)]
pub struct AppState {
  pub counter: u64,
  pub username: String,
}

// The macro generates an enum named `AppStateSlice` with variants derived from fields:
// `Counter`, `Username`, ...
```

Reducer side:

- Return `StateChange::Infer` to have Oxide infer which slices changed.
- Return `StateChange::Slices(&[...])` when you already know which slices changed.
- `StateChange::Full` always passes slice filters (acts like “full update”).

Flutter side:

```dart
@OxideStore(
  // ...
  slices: [AppStateSlice.counter],
)
class AppOxide {}
```
