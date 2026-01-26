# oxide_runtime

`oxide_runtime` is a small Flutter package that provides Riverpod helpers designed to be used by Oxide-generated code.

This package is intentionally minimal and usage-agnostic.

For runnable usage, see the repository examples and the root README:

- `examples/` (end-to-end apps)
- `flutter/oxide_generator` (how `*.oxide.g.dart` is produced)
- `README.md` at the repo root

## What It Provides

- A standard engine lifecycle wrapper (`OxideEngineController`)
- Helpers to create providers from that engine (`oxideEngineProvider`)
- Helpers for generated stream/future providers that depend on an engine provider

## Usage

```dart
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

final engineProvider = oxideEngineProvider<MyEngine>(
  OxideEngineController(
    create: () async => MyEngine.create(),
    dispose: (engine) async => engine.dispose(),
  ),
);

final valueProvider = oxideStreamFromEngineProvider<MyEngine, int>(
  engineProvider: engineProvider,
  createStream: (engine) => engine.valueStream(),
);
```

For complete, runnable usage, see the repository examples.

## License

MIT. See [LICENSE](./LICENSE).
