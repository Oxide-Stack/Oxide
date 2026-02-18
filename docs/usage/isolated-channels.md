# Oxide Isolated Channels (Feature-Gated)

Oxide isolated channels provide **transport-only** Rust ↔ Dart communication primitives that are:

- Strongly typed (enums across the boundary)
- Deterministically bound (callbacks match by variant name)
- Explicitly initialized (no implicit routing)
- Fully optional via a Cargo feature flag

This feature implements the locked specification in `instructions/OxideIsolatedChannels_Locked.md`.

## Enable The Feature

In your Rust crate that uses Oxide, enable `isolated-channels` on both crates:

```toml
[dependencies]
oxide_core = { version = "0.3.0", features = ["isolated-channels"] }
oxide_generator_rs = { version = "0.3.0", features = ["isolated-channels"] }
```

Then gate your channel declarations so they do not compile unless explicitly enabled:

```rust
#[cfg(feature = "isolated-channels")]
use oxide_core::{OxideCallbacking, OxideEventChannel, OxideEventDuplexChannel};
```

## Initialization (`initOxide` Integration)

Applications should initialize the channel runtime during startup, alongside the normal Oxide runtime initialization.

In your FRB API module:

```rust
#[flutter_rust_bridge::frb]
pub async fn init_oxide() -> Result<(), oxide_core::OxideError> {
  fn thread_pool() -> oxide_core::runtime::ThreadPool {
    crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()
  }

  let _ = oxide_core::runtime::init(thread_pool);

  #[cfg(feature = "isolated-channels")]
  oxide_core::init_isolated_channels()?;

  Ok(())
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

The macro also generates a FRB stream endpoint that emits `AnalyticsEvent` values. For a stable FRB surface, wrap the generated endpoint in your own API module (recommended).

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

For each request variant, the macro generates one async method. The response variant must exist with the **same name**, otherwise compilation fails.

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

Outgoing is a Rust → Dart stream (like an event channel). Incoming is Dart → Rust via an FRB function call that forwards the typed `ChatIn` value to the Rust-side registered handler.

## Dart Wiring Helpers (Optional)

The Oxide runtime package includes optional helpers under:

`package:oxide_runtime/src/isolated_channels/isolated_channels.dart`

These helpers are not exported from the main `oxide_runtime` library so apps opt in explicitly.
