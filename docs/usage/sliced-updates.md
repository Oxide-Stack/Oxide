# Sliced Updates

Sliced updates help large screens stay responsive by rebuilding only when relevant parts of state changed.

Use this when your state has several top-level sections and different widgets care about different sections.

## How It Works

1. Rust reducer mutates state.
2. Reducer returns a `StateChange`.
3. Oxide emits a snapshot with a `slices` list.
4. Flutter generated adapters compare `snapshot.slices` to your configured store slices.
5. Widgets rebuild only when there is a match.

## Rust Setup

Enable slice generation on your state:

```rust
use oxide_generator_rs::state;

#[state(sliced = true)]
pub struct AppState {
  pub session: SessionState,
  pub todos: Vec<TodoItem>,
  pub settings: SettingsState,
}
```

This generates a top-level slice enum (for example, `AppStateSlice`) with one variant per top-level field.

## Choosing The Right `StateChange`

Use one of these return values from `reduce` or `effect`:

- `StateChange::None`: no commit and no snapshot.
- `StateChange::Full`: commit and broadcast to all listeners.
- `StateChange::Infer`: commit and let Oxide detect changed top-level fields.
- `StateChange::Slices(&[...])`: commit and explicitly declare changed slices.

Practical rule:

- Use `Infer` when reducer logic is straightforward and you want less manual bookkeeping.
- Use `Slices(&[...])` when you already know exactly what changed and want explicit control.

## Flutter Setup

Configure slice filtering in your store declaration:

```dart
@OxideStore(
  state: AppState,
  snapshot: AppStateSnapshot,
  actions: AppAction,
  engine: ArcAppEngine,
  slices: [AppStateSlice.todos],
)
class AppOxide {}
```

Widgets consuming this store update only for:

- full snapshots, or
- snapshots where `todos` is included in `snapshot.slices`.

## Notes And Limits

- Slices are based on top-level fields, not nested paths.
- If multiple fields changed, include all relevant slices when returning `StateChange::Slices`.
- `StateChange::Full` bypasses slice filtering by design.

## Where To See It In Action

- Counter and benchmark examples for end-to-end generated store flow.
- Reducer API reference: [reducer-pattern.md](./reducer-pattern.md)
- Store declaration reference: [declare-store.md](./declare-store.md)
