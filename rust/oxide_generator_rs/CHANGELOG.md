## 0.4.0
- Add navigation & route generation: support attribute-driven route definitions via a new `#[oxide_route]`-style surface and generate a unified `oxide.dart` entrypoint that exports the runtime and generated routes.
- Emit initialization scaffolding to support the unified `OxideStack.init()` pattern and simplify app-level startup wiring (replaces prior `RustLib.init()` + `initOxide()` flows in generated outputs).
- Generate isolated-channel codegen (event, callback, duplex) and helper bindings so runtime primitives are wired automatically from codegen output.
- Update FRB codegen configuration to support `rust_input: crate` for broader module scanning, improving discovery of reducers/routes across workspace crates.
- Refactor codegen internals for clearer SRP separation and more maintainable modular code generation stages (scan, validate, naming, emit).
- Improve generator metadata and diagnostics for navigation and channel generation; add recommendations in emitted comments for migration steps.

## 0.3.0
- Add isolated channels codegen pipeline (scan, validate, naming, event/callback generation) and compile tests
- Add navigation routes codegen
- Improve reducer expansion for sliced-state inference output
- Refresh UI test expected stderr outputs and update metadata

## 0.2.0
- Split reducer macro implementation into focused modules
- Generate state-specific slice enums (e.g. AppStateSlice) instead of StateSlice
- Detect Infer/Slices usage via AST and add #[reducer(sliced = ...)] override
- Drop tokio_handle/runtime-handle based codegen; engine constructors are async and use InitContext
- Introduce #[state(sliced = true)] for struct states to generate slice enums and fieldwise inference
- Add compile-time validation to prevent sliced enum usage and ensure compatibility