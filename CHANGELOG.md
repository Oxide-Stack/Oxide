## 0.5.0
- Runtime and persistence:
  - Updated persistence to always use bincode with optional validated debug JSON copies.
  - Added Dart-to-Rust debug JSON persistence toggle wiring through `OxideStack.init` and debug logging flags.
  - Added structured runtime logging for initialization, dispatch, and error paths.
  - Refactored reducer engine internals with `ReducerCtx` signatures and clearer engine state naming.
  - Refactored initialization and navigation startup handling; relaxed init validation to reduce false positives.
- Tooling and code generation:
  - Updated navigation codegen to use web-safe FRB imports for isolated channels.
  - Mapped route update payload generation through `routes.dart` bindings.
  - Added FRB guard helpers for isolated channels.
  - Added optional `#[routes(...)]` hooks with signature validation for init/route-change callbacks.
  - Added implicit routes module discovery, FRB metadata checks, and auto-derived standard traits for `#[oxide_route]` types.
  - Fixed version sync to preserve original line endings.
- Tests and QA:
  - Added persistence test validating debug JSON decodes to the same state as bincode snapshots.
  - Added coverage for isolated channels, reducer arguments, persistence worker, and navigation runtime behavior.
  - Added modular QA pipeline spanning Rust crates, Flutter packages, and example suites.
- CI:
  - Added Test Suite workflow for full QA on pull requests across Linux and Windows.
  - Updated Basic CI to run minimal QA on pushes/dispatch and ignore tag builds.
  - Updated release workflow with tag/version alignment checks, multi-platform QA, and publish gates.
  - Updated workflow triggers: Basic CI skips `main` pushes, Test Suite runs on PRs, releases run on tag pushes.
- Examples:
  - Added `showcase_app` Flutter + Rust example and bumped its versions to `0.4.0`.
  - Added an isolated channels demo with related routes.
  - Updated example bridges to expose `setPersistenceDebugJsonEnabled`.
- Docs and maintenance:
  - Updated persistence docs for bincode-only storage with debug JSON copy behavior.
  - Added web notes for isolated channels docs, including FRB web binding guidance.
  - Added sliced updates docs for state slice snapshots and widget rebuild filtering.
  - Updated README/usage example lists to include `showcase_app`.
  - Added a root GitHub Actions overview and a root repo architecture guide.
  - Refactored documentation and code comments for clarity and consistency.

## 0.4.0
- Add navigation and isolated channels support across Rust and Flutter packages; unify initialization with `OxideStack.init()`
- Introduce the `oxide_route` macro and generator improvements to produce a unified `oxide.dart` entry point and typed route command streams
- Integrate navigation and route generation into codegen: attribute-driven route definitions and generated route bindings
- Generate isolated-channel runtime and event/duplex helpers automatically and wire them between codegen and runtimes
- Update FRB codegen configuration to support `rust_input: crate` for broader module scanning and improved generated entrypoints
- Emit initialization scaffolding to support unified `OxideStack.init()` and migration from older `RustLib.init()` / `initOxide()` patterns
- Navigator 1.0 integration and confirm-route handling; improved GoRouter handler and route stack synchronization
- Add `OxideNavigationState` and `OxideNavigationError`, ValueNotifier-based navigation state management, disposal/lifecycle handling, and error hooks for command stream/navigation failures
- Refactor code generation and runtime internals for improved structure, single-responsibility separation, and maintainability
- Update examples (counter, todos, ticker, benchmark, api_browser) with navigation routes, isolated-channel demos, and migration to the new init pattern
- Tooling/workflow: run FRB codegen checks in CI, regenerate FRB outputs in QA scripts, include `api_browser_app` in release tests, and fail when `frb_generated` is stale
- Documentation: add a Navigation Migration Guide and refresh usage docs for isolated channels, navigation, reducer patterns, persistence, and UI backends
- Bugfixes and small improvements:
  - Ignore `popUntil` when the target route kind is missing
  - Add a minimal `oxide_stack` stub for runtime tests
  - Align example build scripts and improve Windows Dart SDK detection in `rust_builder`

## 0.3.0
- Add isolated channels and navigation runtime support across Rust and Flutter packages
- Add isolated channels and navigation codegen support in Rust/Flutter generators and annotations
- Expand example apps with navigation routes, isolated-channel demos, and routing benchmarks
- Improve `rust_builder` Windows Dart SDK detection and align example build scripts
- Refresh usage docs for isolated channels, navigation, reducer patterns, persistence, and UI backends
- Update workspace metadata and root docs for the 0.3.0 release

## 0.2.0
- Add sliced state updates and partial rebuild support across runtimes
- Add architecture overview and dependency graphs for new contributors
- Document sliced updates and snapshot slices semantics across Rust and Flutter
- Replace Tokio runtime wiring docs with `initOxide`-based initialization
- Add `api_browser_app` example that browses a JSON API with multiple reducers and refactor runtime initialization
- Refactor existing examples to use the `oxide_core::runtime::init` pattern
- Remove per-app Tokio runtime modules and update FFI exports accordingly
- Update build and QA scripts to include the new example
- Update `todos_app` and `ticker_app` examples to use sliced updates with inference
- Improve documentation with feature-focused guides and sliced updates explanation
- Add test coverage for slice filtering and sliced state generation

## 0.1.1
- Upgrade `flutter_riverpod` to ^3.2.0 across all examples and runtime
- Update `analyzer`, `build`, `source_gen`, and lints to latest versions in `oxide_generator`
- Fix unit struct handling in Rust reducers (change `AppReducer;` to `AppReducer {}`)
- Improve generated Riverpod provider syntax to match Riverpod 3.x patterns
- Add comprehensive documentation about FRB compatibility and error handling
- Fix Dart analyzer warnings in `oxide_store_generator`
- Update release workflow to handle version verification and pub.dev publishing
- Rename `CoreError` to `OxideError` and consolidate error types
- Add integration tests for state persistence and controller lifecycle
- Extend QA scripts to run Rust tests and integration tests on detected devices

## 0.1.0
- Initial release of Oxide (Rust engine + Flutter codegen workflow).
- Rust: `oxide_core` store engine primitives and snapshot streaming.
- Rust: `oxide_generator_rs` state/actions/reducer macros (FRB-friendly surface generation).
- Flutter: `oxide_annotations`, `oxide_generator`, and `oxide_runtime`.
- Examples: counter, todos (persistence), ticker, benchmark.

## 0.0.1
- Initial Release
