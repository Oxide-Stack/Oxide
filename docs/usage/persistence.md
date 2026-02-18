# Persistence (Optional)

Persistence is feature-gated in Rust.

Enable it in your Rust dependency:

```toml
[dependencies]
oxide_core = { version = "0.3.0", features = ["state-persistence"] }
```

When working inside this repository, use a combined version + path dependency (Cargo prefers `path` locally, while published crates resolve by `version`):

```toml
oxide_core = { version = "0.3.0", path = "../rust/oxide_core", features = ["state-persistence"] }
```

Then use the persistence options supported by the reducer macro. For a working configuration, see:

- [todos_app](../../examples/todos_app)
