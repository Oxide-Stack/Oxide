# Oxide Usage Guide (Step-by-Step)

Use this page as the **single ordered path** for integrating Oxide in a Flutter + Rust app.

If you follow the steps in order, you should end with:

1. A Rust reducer engine exposed over FRB.
2. A generated Dart store adapter.
3. A Flutter UI wired to Oxide runtime/state updates.

## 0. Start From a Known-Good Base

You need an FRB-integrated app first (or start from an example):

- [counter_app](../../examples/counter_app)
- [todos_app](../../examples/todos_app)
- [ticker_app](../../examples/ticker_app)
- [benchmark_app](../../examples/benchmark_app)
- [api_browser_app](../../examples/api_browser_app)

Reference: FRB docs https://fzyzcjy.github.io/flutter_rust_bridge/

## 1. Define Rust State + Actions + Reducer

Follow: [reducer-pattern.md](./reducer-pattern.md)

At the end of this step, your Rust crate has:

- `#[state]` state model
- `#[actions]` action enum
- `#[reducer(...)] impl Reducer` with `reduce` and `effect`

## 2. Add Optional Reducer Features (If Needed)

Apply only what your app needs:

- Sliced updates: [sliced-updates.md](./sliced-updates.md)
- Persistence: [persistence.md](./persistence.md)
- Navigation routes: [navigation.md](./navigation.md)
- Isolated channels: [isolated-channels.md](./isolated-channels.md)

## 3. Generate/Renew Rust <-> Dart Bindings

Follow: [frb-bindings.md](./frb-bindings.md)

When reducer signatures or exported Rust API changes, regenerate FRB bindings before continuing.

## 4. Configure Flutter-Side Generator Inputs

Follow: [flutter-codegen.md](./flutter-codegen.md)

This wires package dependencies and build-runner setup for Oxide adapter generation.

## 5. Declare the Store Contract in Dart

Follow: [declare-store.md](./declare-store.md)

At the end of this step, `@OxideStore(...)` points to your Rust bridge/model types and selected runtime options.

## 6. Run Dart Code Generation

Use build_runner to generate/update adapter files.

The exact command and expected generated files are covered in [flutter-codegen.md](./flutter-codegen.md).

## 7. Initialize Oxide Before Any Store Usage

Follow: [init-oxide.md](./init-oxide.md)

Rule: call `await OxideStack.init()` during app startup before using generated Oxide APIs.

## 8. Wire the Generated Adapter to UI State Management

Follow: [ui-backends.md](./ui-backends.md)

This binds Oxide state stream and actions to your chosen UI integration path.

## 9. Validate End-to-End Behavior

Minimum checks:

1. App starts with `OxideStack.init()` and no init-time errors.
2. Dispatching actions updates state as expected.
3. Stream/state rebuilds happen in Flutter.
4. Optional features (navigation/persistence/channels) execute correctly.

## 10. Debugging and Deep Runtime Tracing

Use startup/log guidance in [init-oxide.md](./init-oxide.md#logging-usage), including:

- `OXIDE_DEBUG_LOGS=true`
- `ENABLE_ADVANCED_LOGS=true`

For internal pipeline details (macro expansion -> runtime -> generated adapters), see:

- [misc/rust-core-generator-workflow.md](../../misc/rust-core-generator-workflow.md)

## Common Pitfalls Checklist

If integration fails, verify in this order:

1. FRB bindings are regenerated after Rust API changes.
2. Dart build_runner outputs are regenerated after annotation/signature changes.
3. `OxideStack.init()` runs before any Oxide engine/store access.
4. Enabled Rust/Dart features match (navigation/persistence/channels).
5. Generated files are committed and in sync with source macros/annotations.
