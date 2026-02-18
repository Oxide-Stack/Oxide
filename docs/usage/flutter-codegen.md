# Flutter Deps + Codegen

Oxide’s Dart-side integration uses build_runner code generation.

## Add Packages

Add the packages you need in your Flutter app:

- `oxide_annotations` (for `@OxideStore`)
- `oxide_generator` (for build_runner code generation)
- `oxide_runtime` (runtime helpers used by generated code)

In your `pubspec.yaml`:

```yaml
dependencies:
  oxide_annotations: ^0.3.0
  oxide_runtime: ^0.3.0

dev_dependencies:
  build_runner: ^2.4.0
  oxide_generator: ^0.3.0
```

Then run:

```bash
flutter pub get
dart run build_runner build -d
```
