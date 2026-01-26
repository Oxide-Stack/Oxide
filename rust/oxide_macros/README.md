# oxide_macros

`oxide_macros` provides proc-macro attributes used to annotate Oxide state and reducer types.

The macros embed structured metadata as Rust doc strings so that code generation tools can discover:

- state names and field shapes
- action names and variant shapes
- reducer ↔ state/actions associations

This crate is intentionally usage-agnostic. For end-to-end Rust ↔ Flutter wiring, see the repository [examples](../../examples) and the root [README](../../README.md).

## Add It To Your Crate

In your `Cargo.toml`:

```toml
[dependencies]
oxide_macros = "1.0.0"
oxide_core = "1.0.0"
```

When working inside this repository, the examples use path dependencies instead.

## Macros

### `#[state]`

Use on a `struct` (single-state) or `enum` (multi-state).

The macro ensures these derives exist:

- `Debug, Clone, PartialEq, Eq`

Serialization derives (`Serialize`, `Deserialize`) are only added when the `state-persistence` feature is enabled on `oxide_macros`. When enabled, the macro also injects `#[serde(crate = "oxide_core::serde")]` so downstream crates do not need to depend on `serde` directly.

Example (struct state):

```rust
use oxide_macros::state;

#[state]
pub struct AppState {
  pub counter: u64,
}
```

Example (enum state):

```rust
use oxide_macros::state;

#[state]
pub enum SessionState {
  LoggedOut,
  LoggedIn { user_id: String },
}
```

### `#[actions]`

Use on an `enum` representing a reducer’s action set.

The macro injects the same derives and serde crate attribute as `#[state]`.

Example:

```rust
use oxide_macros::actions;

#[actions]
pub enum AppAction {
  Increment,
  SetName { name: String },
}
```

### `#[reducer(...)]`

Use on an `impl oxide_core::Reducer for <Type>` block.

Required arguments:

- `engine = <Ident>`: generated engine wrapper type
- `snapshot = <Ident>`: generated snapshot type
- `initial = <expr>`: initial state expression

Optional arguments:

- `reducer = <expr>`: reducer construction expression (defaults to `Default::default()`)
- `init_app`: emits a Flutter Rust Bridge `init` function (when FRB glue is enabled)
- `no_frb`: disables emitting Flutter Rust Bridge glue for this reducer (useful for pure-Rust reducers)
- `persist = "some.key"` and `persist_min_interval_ms = 200`: enables persistent engine wiring (requires `state-persistence` feature)

The annotated impl must define:

- `type State = ...;`
- `type Action = ...;`
- `type SideEffect = ...;`
- `fn init(&mut self, sideeffect_tx: tokio::sync::mpsc::UnboundedSender<Self::SideEffect>)`
- `fn reduce(&mut self, state: &mut Self::State, action: Self::Action) -> oxide_core::CoreResult<oxide_core::StateChange>`
- `fn effect(&mut self, state: &mut Self::State, effect: Self::SideEffect) -> oxide_core::CoreResult<oxide_core::StateChange>`

Example:

```rust
use oxide_macros::{actions, reducer, state};

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
pub struct AppReducer;

#[reducer(engine = AppEngine, snapshot = AppSnapshot, initial = AppState { counter: 0 }, no_frb)]
impl oxide_core::Reducer for AppReducer {
  type State = AppState;
  type Action = AppAction;
  type SideEffect = AppSideEffect;

  fn init(
    &mut self,
    _sideeffect_tx: oxide_core::tokio::sync::mpsc::UnboundedSender<Self::SideEffect>,
  ) {}

  fn reduce(
    &mut self,
    state: &mut Self::State,
    action: Self::Action,
  ) -> oxide_core::CoreResult<oxide_core::StateChange> {
    match action {
      AppAction::Increment => state.counter = state.counter.saturating_add(1),
    }
    Ok(oxide_core::StateChange::FullUpdate)
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

## Features

- `frb` (default): enables FRB export generation for `#[reducer(...)]` (can still be disabled per reducer via `no_frb`)
- `state-persistence`: enables serde derives injection for `#[state]` / `#[actions]` and enables persistence arguments for `#[reducer(...)]`
- `full`: enables all features

## Metadata

The macros add doc strings containing `oxide:meta:<json>`. The JSON includes user doc comments and structural information needed for generators.

## License

MIT. See [LICENSE](../../LICENSE).
