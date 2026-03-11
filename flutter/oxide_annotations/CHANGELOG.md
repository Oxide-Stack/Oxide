## 0.4.0
- Expand navigation annotations and metadata to support macro-driven route generation and migration to the new navigation init pattern
- Add support for `oxide_route`-style metadata consumed by the generator to produce typed route bindings and a unified `oxide.dart` entrypoint
- Add annotation metadata to enable isolated-channel codegen (event/duplex/callback) and runtime wiring
- Update annotation documentation and examples to align with the unified `OxideStack.init()` initialization pattern

## 0.3.0
- Add navigation annotations support and refresh metadata

## 0.2.0
- No changes in this release.

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