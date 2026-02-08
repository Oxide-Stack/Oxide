# Oxide Usage Guide

Oxide is designed to be integrated into a Flutter app that already uses Flutter Rust Bridge (FRB).

This folder splits the original single-file guide into feature-focused pages.

## Working Examples

For fully working code, follow the examples and mirror their structure:

- [counter_app](../../examples/counter_app)
- [todos_app](../../examples/todos_app)
- [ticker_app](../../examples/ticker_app)
- [benchmark_app](../../examples/benchmark_app)
- Benchmarks write-up: [BENCHMARKS.md](../BENCHMARKS.md)

## Prerequisites

Start from an FRB-integrated Flutter app/template (or copy one of this repo’s examples). Oxide assumes you already have a working FRB setup for:

- building Rust
- generating bindings
- initializing `RustLib` in Flutter

FRB docs: https://fzyzcjy.github.io/flutter_rust_bridge/

## Topics

- Reducer pattern (Rust state/actions/reducer): [reducer-pattern.md](./reducer-pattern.md)
- Generate FRB bindings (Rust ↔ Dart): [frb-bindings.md](./frb-bindings.md)
- Unified async initialization (`initOxide`): [init-oxide.md](./init-oxide.md)
- Flutter deps + codegen (`build_runner`): [flutter-codegen.md](./flutter-codegen.md)
- Declare a store (`@OxideStore`): [declare-store.md](./declare-store.md)
- Use the generated adapter in UI: [ui-backends.md](./ui-backends.md)
- Persistence (optional): [persistence.md](./persistence.md)

