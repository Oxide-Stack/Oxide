

## 0.3.0
- Add isolated channels and navigation runtime support across Rust and Flutter packages
- Add isolated channels and navigation codegen support in Rust/Flutter generators and annotations
- Expand example apps with navigation routes, isolated-channel demos, and routing benchmarks
- Improve rust_builder Windows Dart SDK detection and align example build scripts
- Refresh usage docs for isolated channels, navigation, reducer patterns, persistence, and UI backends
- Update workspace metadata and root docs for the 0.3.0 release

## 0.2.0
- Add sliced state updates and partial rebuild support across runtimes
- Add architecture overview and dependency graphs for new contributors
- Document sliced updates and snapshot slices semantics across Rust and Flutter
- Replace Tokio runtime wiring docs with initOxide-based initialization
- Add api_browser_app example that browses a JSON API with multiple reducers and refactor runtime initialization
- Refactor existing examples to use the oxide_core::runtime::init pattern
- Remove per-app Tokio runtime modules and update FFI exports accordingly
- Update build and QA scripts to include the new example
- Update todos_app and ticker_app examples to use sliced updates with inference
- Improve documentation with feature-focused guides and sliced updates explanation
- Add test coverage for slice filtering and sliced state generation

## 0.1.1
- Upgrade flutter_riverpod to ^3.2.0 across all examples and runtime
- Update analyzer, build, source_gen, and lints to latest versions in oxide_generator
- Fix unit struct handling in Rust reducers (change `AppReducer;` to `AppReducer {}`)
- Improve generated Riverpod provider syntax to match Riverpod 3.x patterns
- Add comprehensive documentation about FRB compatibility and error handling
- Fix Dart analyzer warnings in oxide_store_generator
- Update release workflow to handle version verification and pub.dev publishing
- Rename CoreError to OxideError and consolidate error types
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
