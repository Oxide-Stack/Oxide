# Reducer Pattern (Rust, Fool-Proof Setup)

This guide gives the exact Rust steps for a working Oxide reducer pipeline.

## 1. Add Crate Dependencies

In your Rust crate (`Cargo.toml`):

```toml
[dependencies]
oxide_core = "0.4.0"
oxide_generator_rs = "0.4.0"
```

Inside this monorepo, prefer version + path together:

```toml
oxide_core = { version = "0.4.0", path = "../rust/oxide_core" }
oxide_generator_rs = { version = "0.4.0", path = "../rust/oxide_generator_rs" }
```

## 2. Define Your State

Annotate the state model with `#[state]`:

```rust
#[state]
pub struct AppState {
  pub counter: u64,
}
```

## 3. Define UI/System Inputs as Actions

Annotate your action enum with `#[actions]`:

```rust
#[actions]
pub enum AppAction {
  Increment,
}
```

## 4. Define Side-Effects Channel Type

Even if you do not emit effects yet, define the side-effect type:

```rust
pub enum AppSideEffect {}
```

## 5. Implement `Reducer` and Annotate with `#[reducer(...)]`

This is the minimum complete pattern:

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
    ctx: oxide_core::ReducerCtx<'_, Self::Action, Self::State>,
  ) -> oxide_core::CoreResult<oxide_core::StateChange> {
    match ctx.input {
      AppAction::Increment => state.counter = state.counter.saturating_add(1),
    }
    Ok(oxide_core::StateChange::Full)
  }

  fn effect(
    &mut self,
    _state: &mut Self::State,
    _ctx: oxide_core::ReducerCtx<'_, Self::SideEffect, Self::State>,
  ) -> oxide_core::CoreResult<oxide_core::StateChange> {
    Ok(oxide_core::StateChange::None)
  }
}
```

## 6. Understand What Is Generated

The macros generate (at minimum):

- Engine type (`engine = ...`).
- Snapshot type (`snapshot = ...`).
- FRB-friendly exports by default (`no_frb` disables this behavior).

For full macro/runtime pipeline details, see:

- [../../misc/rust-core-generator-workflow.md](../../misc/rust-core-generator-workflow.md)

## 7. Return the Correct `StateChange`

`StateChange` controls commit + emission behavior:

- `StateChange::None`: no commit, no new snapshot.
- `StateChange::Full`: commit and emit full update.
- `StateChange::Infer`: infer changed slices (sliced mode).
- `StateChange::Slices(&[...])`: explicitly mark changed slices.

## 8. Optional: Enable Sliced Updates

For targeted Flutter rebuilds:

1. Add `#[state(sliced = true)]`.
2. Return `Infer` or `Slices(...)`.
3. Configure matching slices in `@OxideStore`.

Details: [sliced-updates.md](./sliced-updates.md)

## 9. Done Criteria (Before Moving to Dart)

Before generating Dart adapters, verify:

1. Rust reducer compiles with your selected features.
2. Reducer paths always return a valid `CoreResult<StateChange>`.
3. Initial state and snapshot names in `#[reducer(...)]` are correct.
