## 0.4.0
- Unified initialization: add `OxideStack.init()` as a single initialization point replacing older `RustLib.init()` + `initOxide()` flows
- Navigator 1.0 integration and confirm-route handling: improved GoRouter handler, route stack synchronization, and fail-fast behavior for navigation command timeouts
- Enhanced navigation runtime surface:
  - Introduce `OxideNavigationState` and `OxideNavigationError` types for clearer error handling and typed route state
  - ValueNotifier-based navigation state management with disposal and lifecycle hooks
  - Error hooks for command-stream and navigation failures; errors are surfaced via Zone for better observability
- Codegen/runtime integration:
  - Support for macro-driven route generation (via `oxide_route` metadata) and typed FRB command streams for route payloads
  - Emit and support isolated-channel runtime helpers (events, callbacks, duplex) used by codegen
  - Added a minimal `oxide_stack` stub used by runtime tests and generated entry points
- Bugfixes:
  - Ignore `popUntil` when the target route kind is missing to avoid unexpected route errors
- Docs & migration:
  - Add Navigation Migration Guide and update runtime docs to show migration from manual bindings to macro-driven APIs and `OxideStack.init()`
  - Update examples to use the new initialization pattern and generated `oxide.dart` entrypoint

## 0.3.0
- Add isolated channels Dart runtime (event, callback, duplex) and runtime tests
- Add navigation runtime APIs and a go_router handler

## 0.2.0
- Split runtime core into SRP-focused files while keeping exports stable
- Implement `filterSnapshotsBySlices` in Dart runtime for efficient partial rebuilds
- Support slice filtering in all Flutter backends (InheritedWidget, Riverpod, BLoC)

## 0.1.1
- Upgrade `flutter_riverpod` to ^3.2.0 across all examples and runtime
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
- Initial release of Oxide Flutter runtime and integration pieces
- Dart runtime primitives for snapshot filtering, persistence hooks, and basic navigation helpers

## 0.0.1
- Initial Release