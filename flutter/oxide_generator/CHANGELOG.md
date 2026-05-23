## 0.5.0
- Updated navigation codegen to use web-safe FRB imports for isolated channels.
- Mapped route update payload generation through `routes.dart` bindings.
- Added FRB guard helpers for isolated-channel generation paths.
- Updated version sync to preserve original line endings when updating files.

## 0.4.0
- Integrate oxide generator for navigation and route definitions:
  - Add `#[oxide_route]` macro support and generation of typed route bindings
  - Emit a unified `oxide.dart` entry point that exports runtime helpers and generated routes
- Generate isolated-channel helpers and event/duplex bindings from codegen to wire runtime primitives automatically
- Update FRB codegen configuration to support `rust_input: crate` for broader module scanning and more accurate generation
- Emit initialization scaffolding and patterns to support unified `OxideStack.init()` initialization across Flutter and native entrypoints
- Refactor generated code structure for improved consistency, maintainability, and SRP alignment
- Improve generator metadata and documentation to assist migration from manual route implementations

## 0.3.0
- Add navigation codegen builder and refresh metadata

## 0.2.0
- Split string-based codegen into SRP-focused modules without changing output
- Make `OxideCodegenConfig` slice fields optional when unused

## 0.1.1
- Upgrade flutter_riverpod to ^3.2.0 across all examples and runtime
- Update analyzer, build, source_gen, and lints to latest versions in oxide_generator
- Fix unit struct handling in Rust reducers (change `AppReducer;` to `AppReducer {}`)
- Improve generated Riverpod provider syntax to match Riverpod 3.x patterns
- Add comprehensive documentation about FRB compatibility and error handling
- Fix Dart analyzer warnings in oxide_store_generator
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
