# Navigation (Rust-driven)

Oxide navigation is a feature-gated routing layer driven from Rust and wired into Flutter navigation backends (Navigator 1.0 and GoRouter).

Reducers and effects decide where to go. Flutter keeps the navigation plumbing.

## Enable the Feature

### Rust

Enable the feature on `oxide_core` (and forward it from your app crate):

```toml
[dependencies]
oxide_core = { path = "../../rust/oxide_core", features = ["navigation-binding"] }
oxide_generator_rs = { path = "../../rust/oxide_generator_rs", features = ["navigation-binding"] }

[features]
navigation-binding = ["oxide_core/navigation-binding", "oxide_generator_rs/navigation-binding"]
```

### Dart

Add `oxide_annotations`, `oxide_generator`, and `oxide_runtime` as you already do for store codegen. Navigation generation applies to dependents and produces `lib/oxide_generated/...`.

## Define Routes in Rust

Create a `routes/` module and apply `#[oxide_generator_rs::routes]` to the module item:

```rust
#[cfg(feature = "navigation-binding")]
#[oxide_generator_rs::routes]
pub mod routes {
    include!("routes/mod.rs");
}
```

Each route is a Rust struct annotated with `#[oxide_generator_rs::oxide_route(...)]`:

```rust
#[oxide_generator_rs::oxide_route()]
pub struct SplashRoute {}
```

`#[oxide_route]` automatically adds `Clone`, `Debug`, `serde::Serialize`, and `serde::Deserialize` derives.

The macro scans `src/routes/` and writes a JSON metadata file to `target/oxide_routes/`. The Dart generator reads that file.

When `navigation-binding` is enabled, the macro generates FRB-ready navigation endpoints under `crate::routes::oxide_navigation` (for example: `init_navigation`, `oxide_nav_commands_stream`, `oxide_nav_emit_result`, `oxide_nav_set_current_route`). Do not hand-write these bindings; the examples re-export them through `crate::api::oxide_navigation` for FRB discovery.

## Bind Routes to Widgets in Dart

Annotate your app widget and page widgets:

```dart
@OxideApp(navigation: OxideNavigation.navigator())
class MyApp extends StatefulWidget { ... }

@OxideApp(navigation: OxideNavigation.goRouter())
class MyRouterApp extends StatefulWidget { ... }

@OxideRoutePage(RouteKind.splash)
final class SplashScreen extends StatelessWidget { ... }

@OxideRoutePage(RouteKind.home)
final class HomeScreen extends StatelessWidget { ... }
```

Then import the single public entrypoint:

```dart
import 'oxide.dart';
```

## Execute Rust Commands in Dart

Navigation runtime setup is generated.

Use the generated navigator key with your navigation backend (Navigator 1.0 or GoRouter), and call `OxideStack.init()` from `main()`. If you use `runOxideApp(..., startNavigation: true)` or pass `startNavigation: true` to `OxideStack.init(...)`, the runtime starts automatically.

```dart
import 'package:go_router/go_router.dart';

import 'oxide.dart';

final router = GoRouter(
  navigatorKey: OxideStack.navigatorKey,
  routes: <RouteBase>[/* your normal GoRouter config */],
);
```

## Notes

- Navigation is feature-gated by `navigation-binding`. Builds without this feature exclude all navigation code.
- Examples in this repository demonstrate a splash-first flow and a Rust-driven transition to the primary route.
- See [navigation-migration.md](./navigation-migration.md) for upgrades from the manual binding model.
