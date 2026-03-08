import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

// expose the generated OxideStack helper through the app-facing library
export '../oxide_generated/oxide_stack.g.dart' show OxideStack;

import 'rust/api/comments_bridge.dart' as comments_api;
import 'rust/api/comments_bridge.dart' show ArcCommentsEngine, CommentsStateSnapshot;
import 'rust/api/posts_bridge.dart' as posts_api;
import 'rust/api/posts_bridge.dart' show ArcPostsEngine, PostsStateSnapshot;
import 'rust/api/users_bridge.dart' as users_api;
import 'rust/api/users_bridge.dart' show ArcUsersEngine, UsersStateSnapshot;
// helper methods for configuring the HTTP client base URL
import 'rust/api/bridge.dart' as api;

// re-export channel bridge APIs and types to keep example code simple
export 'package:api_browser_app/src/rust/api/isolated_channels_bridge.dart';
export 'package:api_browser_app/src/rust/isolated_channels_demo/channels.dart';

// private alias for bridge wrappers
// ignore: no_leading_underscores_for_library_prefixes
import 'package:api_browser_app/src/rust/api/isolated_channels_bridge.dart' as _ch;

// channel types needed for wrapper signatures
import 'package:api_browser_app/src/rust/isolated_channels_demo/channels.dart';

import 'rust/state/comments_action.dart';
import 'rust/state/comments_state.dart';
import 'rust/state/posts_action.dart';
import 'rust/state/posts_state.dart';
import 'rust/state/users_action.dart';
import 'rust/state/users_state.dart';

part 'oxide.oxide.g.dart';

// channel type aliases
typedef ApiBrowserDemoDialogPendingRequest = _ch.ApiBrowserDemoDialogPendingRequest;

// convenience wrappers mirroring the bridge API

/// Initialize the demo runtime.
/// Send a notification from Rust to Dart.
Future<void> emitApiBrowserDemoNotification({required String message}) => _ch.emitApiBrowserDemoNotification(message: message);

/// Stream of demo events (Rust → Dart).
///
/// **Deprecated**: use `OxideStack.events.apiBrowserDemoEvents` instead.
@Deprecated('use OxideStack.events.apiBrowserDemoEvents')
Stream<ApiBrowserDemoEvent> apiBrowserDemoEventsStream() => _ch.apiBrowserDemoEventsStream();

/// Stream of callback requests (Rust → Dart).///
/// **Deprecated**: use `OxideStack.callbacks.apiBrowserDemoDialogRequests`.
@Deprecated('use OxideStack.callbacks.apiBrowserDemoDialogRequests')
Stream<_ch.ApiBrowserDemoDialogPendingRequest> apiBrowserDemoDialogRequestsStream() => _ch.apiBrowserDemoDialogRequestsStream();

/// Send a callback response (Dart → Rust).
///
/// **Deprecated**: use `OxideStack.callbacks.apiBrowserDemoDialogRespond`.
@Deprecated('use OxideStack.callbacks.apiBrowserDemoDialogRespond')
Future<void> apiBrowserDemoDialogRespond({required BigInt id, required ApiBrowserDemoDialogResponse response}) =>
    _ch.apiBrowserDemoDialogRespond(id: id, response: response);

/// Request confirmation from Dart.
Future<bool> apiBrowserDemoDialogConfirm({required String title}) => _ch.apiBrowserDemoDialogConfirm(title: title);

/// Duplex helpers
Stream<ApiBrowserDemoOut> apiBrowserDemoDuplexOutgoingStream() => _ch.apiBrowserDemoDuplexOutgoingStream();
Future<void> apiBrowserDemoDuplexSend({required String text}) => _ch.apiBrowserDemoDuplexSend(text: text);
Future<void> apiBrowserDemoDuplexIncoming({required ApiBrowserDemoIn event}) => _ch.apiBrowserDemoDuplexIncoming(event: event);
Future<String?> apiBrowserDemoLastIncomingText() => _ch.apiBrowserDemoLastIncomingText();

/// -----------------------------------------------------------------
/// Example-specific helpers (moved from tests/bridge to public API)
/// -----------------------------------------------------------------

/// Configure the base URL used by the Rust backend HTTP client.
///
/// Exposed here so tests and example code don't import the internal
/// `rust/api/bridge.dart` file directly. This helper simply forwards
/// to the underlying generated bridge method.
Future<void> setApiBaseUrl({required String url}) => api.setApiBaseUrl(url: url);

/// Reset the API base URL to the default shipped value.
Future<void> resetApiBaseUrl() => api.resetApiBaseUrl();

@OxideStore(
  state: UsersState,
  snapshot: UsersStateSnapshot,
  actions: UsersAction,
  engine: ArcUsersEngine,
  backend: OxideBackend.riverpod,
  bindings: 'users_api',
  keepAlive: true,
)
class UsersRiverpodOxide {}

@OxideStore(
  state: PostsState,
  snapshot: PostsStateSnapshot,
  actions: PostsAction,
  engine: ArcPostsEngine,
  backend: OxideBackend.riverpod,
  bindings: 'posts_api',
  keepAlive: true,
)
class PostsRiverpodOxide {}

@OxideStore(
  state: CommentsState,
  snapshot: CommentsStateSnapshot,
  actions: CommentsAction,
  engine: ArcCommentsEngine,
  backend: OxideBackend.riverpod,
  bindings: 'comments_api',
  keepAlive: true,
)
class CommentsRiverpodOxide {}
