## 0.5.0
- Updated persistence to always use bincode, with optional validated debug JSON copies for readable snapshots.
- Refactored reducer engine internals to `ReducerCtx`-based reducer signatures and clearer engine state naming.
- Added test coverage for persistence debug JSON equivalence and related runtime behaviors.

## 0.4.0
- Unified initialization: converge runtime init APIs to enable `OxideStack.init()` replacing the previous `RustLib.init()` + `initOxide()` pattern
- Add typed FRB command stream plumbing used by navigation route payloads and expose initialization hooks for the navigation runtime
- Wire isolated-channel runtime primitives (event, callbacking, duplex) so codegen-generated entrypoints can interoperate with the runtime; include helpers and init plumbing
- Provide runtime migration support for FRB-generated entrypoints and helpers to ease migration from older init patterns
- Internal runtime refactors to improve lifecycle management, initialization ordering, and testability

## 0.3.0
- Add isolated channels runtime primitives (event, callbacking, duplex) with init plumbing and tests
- Add navigation runtime/context, route models, typed route traits, and navigation runtime tests
- Adjust persistence path handling and expand test coverage
- Remove unused store import

## 0.2.0
- Reorganize internals under engine/ while preserving public API
- Reducer init is now async fn init(&mut self, ctx: InitContext<SideEffect>)
- Engines must be created after oxide_core::runtime::init(...) (exposed via app-level initOxide() FRB API)
- Unified FRB-based runtime for store task spawning (native/web/WASI aware)
- Build for wasm32-unknown-unknown and wasm32-wasip1 with all features
- Add web persistence backend (localStorage + base64) for state persistence
- Replace persistence OS thread with a debounced async worker
- WASM-safe background spawning (web uses spawn_local; WASI uses a runtime handle)
- Add StateChange::Infer and StateChange::Slices variants for partial updates
- Extend StateSnapshot with slices field to carry slice metadata
