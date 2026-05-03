# Persistence (Optional)

Persistence is feature-gated in Rust.

Enable it in your Rust dependency:

```toml
[dependencies]
oxide_core = { version = "0.4.0", features = ["state-persistence"] }
```

When working inside this repository, use a combined version + path dependency (Cargo prefers `path` locally, while published crates resolve by `version`):

```toml
oxide_core = { version = "0.4.0", path = "../rust/oxide_core", features = ["state-persistence"] }
```

By default, persistence uses Oxide's binary codec path (bincode). JSON persistence is opt-in and only enabled when you explicitly add `persistence-json`.

For state/action types declared with Oxide macros (`#[state]`, `#[actions]`), serde derives are injected automatically when `state-persistence` is enabled. For nested custom structs/enums referenced by your persisted state, prefer declaring those with `#[state]` too so you don't need manual serde derives.

Then use the persistence options supported by the reducer macro. For a working configuration, see:

- [todos_app](../../examples/todos_app)
