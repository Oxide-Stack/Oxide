# Oxide Isolated Channels (Feature-Gated)

Oxide isolated channels provide **transport-only** Rust ↔ Dart communication primitives that are:

- Strongly typed (enums across the boundary)
- Deterministically bound (callbacks match by variant name)
- Explicitly initialized (no implicit routing)
- Fully optional via a Cargo feature flag

This feature follows the locked specification in `instructions/OxideIsolatedChannels_Locked.md`.

## Enable the Feature

In your Rust crate that uses Oxide, enable `isolated-channels` on both crates:

```toml
[dependencies]
oxide_core = { version = "0.4.0", features = ["isolated-channels"] }
oxide_generator_rs = { version = "0.4.0", features = ["isolated-channels"] }
```

Then gate your channel declarations so they do not compile unless explicitly enabled:

```rust
#[cfg(feature = "isolated-channels")]
use oxide_core::{OxideCallbacking, OxideEventChannel, OxideEventDuplexChannel};
```

## Initialization (`OxideStack.init`)

Initialize Oxide once during startup using the generated entrypoint. The macro-generated Rust init hook runs during `RustLib.init()` and initializes the isolated channel runtime when the feature is enabled.

Call the Dart entrypoint from `main()`:

```dart
Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await OxideStack.init();
  runApp(const MyApp());
}
```

## Rust → Dart: Event Channels

Declare a Rust → Dart event channel:

```rust
pub struct AnalyticsChannel;

#[oxide_generator_rs::oxide_event_channel]
impl oxide_core::OxideEventChannel for AnalyticsChannel {
  type Events = AnalyticsEvent;
}

pub enum AnalyticsEvent {
  Track { name: String },
  ScreenView { screen: String },
}
```

The macro generates strongly typed send helpers:

```rust
AnalyticsChannel::track("signup".to_string());
AnalyticsChannel::screen_view("home".to_string());
```

The macro also generates an FRB stream endpoint that emits `AnalyticsEvent` values. For a stable FRB surface, wrap the generated endpoint in your own API module.

## Rust → Dart → Rust: Callbacking

Declare a callback service:

```rust
pub struct DialogService;

#[oxide_generator_rs::oxide_callback]
impl oxide_core::OxideCallbacking for DialogService {
  type Request = DialogRequest;
  type Response = DialogResponse;
}

pub enum DialogRequest {
  ShowAlert { title: String },
  Confirm { title: String, message: String },
}

pub enum DialogResponse {
  ShowAlert(bool),
  Confirm(bool),
}
```

For each request variant, the macro generates one async method. The response variant must exist with the **same name** or compilation fails.

## Duplex: Paired Independent Streams

Declare a duplex channel:

```rust
pub struct ChatChannel;

#[oxide_generator_rs::oxide_event_channel]
impl oxide_core::OxideEventDuplexChannel for ChatChannel {
  type Outgoing = ChatOut;
  type Incoming = ChatIn;
}

pub enum ChatOut {
  Send { text: String },
}

pub enum ChatIn {
  Receive { text: String },
}
```

Outgoing is a Rust → Dart stream (like an event channel). Incoming is Dart → Rust through an FRB function call that forwards the typed `ChatIn` value to the Rust-side registered handler.

## Dart Wiring Helpers (Optional)

The Oxide runtime package includes optional helpers under:

`package:oxide_runtime/src/isolated_channels/isolated_channels.dart`

These helpers are not exported from the main `oxide_runtime` library, so apps opt in explicitly.

Example callback loop wiring:

```dart
import 'package:oxide_runtime/src/isolated_channels/callback_runtime.dart';

Future<void> startDialogLoop(
  Stream<DialogRequestEnvelope> requests,
) {
  return runOxideCallbacking<DialogRequestEnvelope, DialogRequest, DialogResponse>(
    requests: requests,
    requestIdOf: (e) => e.requestId,
    requestOf: (e) => e.request,
    handler: handleDialogRequest,
    respond: sendDialogResponse,
  );
}
```

Example duplex outgoing listener:

```dart
import 'package:oxide_runtime/src/isolated_channels/duplex_runtime.dart';

StreamSubscription<ChatOut> bindOutgoing(
  Stream<ChatOut> outgoing,
) {
  return listenOxideDuplexOutgoing<ChatOut>(
    outgoing: outgoing,
    onEvent: (event) {
      // Forward into your app service layer.
    },
  );
}
```

If you want concrete end-to-end behavior, see `flutter/oxide_runtime/test/isolated_channels_runtime_test.dart` and the isolated-channel demo in `examples/api_browser_app`.
