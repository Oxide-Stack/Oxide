# Navigation (Rust-driven)

Oxide navigation is a feature-gated, Rust-driven routing layer that integrates with Flutter-native navigation backends (Navigator 1.0 today).

The goal is to let reducers/effects decide *where to go* while keeping all Flutter-specific navigation details in Dart.

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

Add `oxide_annotations`, `oxide_generator`, and `oxide_runtime` as you already do for store codegen. Navigation generation is auto-applied to dependents and produces `lib/oxide_generated/...`.

## Define Routes in Rust

Create a `routes/` module and apply `#[oxide_generator_rs::routes]` to the module item:

```rust
#[cfg(feature = "navigation-binding")]
#[oxide_generator_rs::routes]
pub mod routes {
    include!("routes/mod.rs");
}
```

Each route is a Rust struct implementing `oxide_core::navigation::Route`:

```rust
use oxide_core::navigation::{NoExtra, NoReturn, Route};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SplashRoute;

impl Route for SplashRoute {
    type Return = NoReturn;
    type Extra = NoExtra;
}
```

The macro scans `src/routes/` and writes a JSON metadata file to `target/oxide_routes/`. The Dart generator consumes this file.

When `navigation-binding` is enabled, the macro also generates FRB-ready navigation endpoints under `crate::navigation::frb` (for example: `init_navigation`, `oxide_nav_commands_stream`, `oxide_nav_emit_result`, `oxide_nav_set_current_route`). Applications should not hand-write these bindings.

## Bind Routes to Widgets in Dart

Annotate your app widget and page widgets:

```dart
@OxideApp(navigation: OxideNavigation.navigator())
class MyApp extends StatefulWidget { ... }

@OxideRoutePage(RouteKind.splash)
final class SplashScreen extends StatelessWidget { ... }

@OxideRoutePage(RouteKind.home)
final class HomeScreen extends StatelessWidget { ... }
```

Then import the generated outputs:

```dart
import 'oxide_generated/navigation/route_builders.g.dart';
import 'oxide_generated/routes/route_kind.g.dart';
import 'oxide_generated/routes/route_models.g.dart';
```

## Execute Rust Commands in Dart

Use the runtime coordinator + a handler (Navigator 1.0):

```dart
import 'src/rust/api/navigation_bridge.dart' as rust;

final handler = NavigatorNavigationHandler<OxideRoute, RouteKind>(
  navigatorKey: navigatorKey,
  kindOf: (r) => r.kind,
  routeBuilders: oxideRouteBuilders,
);

final runtime = OxideNavigationRuntime<OxideRoute, RouteKind>(
  commands: rust.oxideNavCommandsStream().map(decodeNavCommand).whereType(),
  handler: handler,
  emitResult: (ticket, result) => rust.oxideNavEmitResult(ticket: ticket, resultJson: jsonEncode(result)),
  setCurrentRoute: (route) => rust.oxideNavSetCurrentRoute(
    kind: route.kind.name,
    payloadJson: jsonEncode(route.toJson()),
  ),
);

rust.initNavigation();
runtime.start();
```

## Notes

- Navigation is feature-gated by `navigation-binding`. Builds without this feature exclude all navigation code.
- Examples in this repository demonstrate a splash-first flow and a Rust-driven transition to the primary route.
