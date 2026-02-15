// Annotation contracts shared between user code, generator, and generated output.
//
// Why: the generator needs stable metadata to produce predictable glue code.
// How: expose a minimal annotation API with clear backend options.
/// Backend integration strategy used by generated store code.
enum OxideBackend {
  /// Generates an `InheritedNotifier`-based scope widget plus a `ChangeNotifier`
  /// controller.
  ///
  /// This backend is framework-light (no Riverpod/BLoC dependency) and works
  /// well for smaller apps or for embedding Oxide into existing widget trees.
  inherited,

  /// Same as [inherited], but also generates a `use<Store>Oxide()` hook helper.
  ///
  /// This backend requires `flutter_hooks`.
  inheritedHooks,

  /// Generates a Riverpod `NotifierProvider` (or non-autoDispose variant when
  /// `keepAlive: true`) that owns the store lifecycle.
  ///
  /// This backend requires `flutter_riverpod`.
  riverpod,

  /// Generates a `Cubit` plus a `BlocProvider` scope widget.
  ///
  /// This backend requires `bloc` and `flutter_bloc`.
  bloc,
}

/// Annotation describing an Oxide store to generate.
///
/// This is applied to a Dart class that represents your store facade. The
/// generator reads the provided types and method names to produce glue code that
/// connects Flutter code to the underlying engine (often provided by FFI).
///
/// ## Example
/// ```dart
/// import 'package:oxide_annotations/oxide_annotations.dart';
///
/// @OxideStore(
///   state: AppState,
///   snapshot: AppStateSnapshot,
///   actions: AppActions,
///   engine: AppEngine,
/// )
/// class AppStore {}
/// ```
final class OxideStore {
  const OxideStore({
    required this.state,
    required this.snapshot,
    required this.actions,
    required this.engine,
    this.slices,
    this.backend = OxideBackend.inherited,
    this.keepAlive = false,
    this.bindings,
    this.createEngine = 'createEngine',
    this.disposeEngine = 'disposeEngine',
    this.dispatch = 'dispatch',
    this.stateStream = 'stateStream',
    this.current = 'current',
    this.initApp,
    this.encodeCurrentState,
    this.encodeState,
    this.decodeState,
    this.name,
  });

  final Type state;
  final Type snapshot;
  final Type actions;
  final Type engine;

  /// Optional slice subscription configuration for sliced updates.
  ///
  /// When set, the generated store only rebuilds when the incoming snapshot
  /// indicates one of the requested slices changed (or when the snapshot
  /// represents a full update).
  ///
  /// The values must be enum values from the state slice enum generated from
  /// Rust (for example: `StateSlice.playback`).
  final List<dynamic>? slices;

  /// Which Flutter state-management backend to generate for.
  final OxideBackend backend;

  /// Whether the generated store should be kept alive when possible.
  ///
  /// This is primarily useful for tab/page view style UIs where off-screen
  /// subtrees might otherwise be disposed and rebuilt, resetting the store.
  ///
  /// Backend behavior:
  /// - Inherited / Hooks / BLoC: requests keep-alive for the generated scope
  ///   widget so the store instance is not disposed when off-screen.
  /// - Riverpod: generates a non-autoDispose provider so the store is not
  ///   disposed when un-watched.
  ///
  /// Defaults to `false`.
  final bool keepAlive;

  /// Optional import alias used to qualify default binding function names.
  ///
  /// This is meant for multi-engine apps where multiple FRB binding libraries
  /// expose the same method names (e.g. `createEngine`, `dispatch`).
  ///
  /// When provided, and you do not override the corresponding string fields,
  /// the generator prefixes defaults with this value. For example, with
  /// `bindings: 'counter_api'`, the default `createEngine` becomes
  /// `counter_api.createEngine`.
  final String? bindings;

  /// Engine creation method name on the bindings object.
  final String createEngine;

  /// Engine disposal method name on the bindings object.
  final String disposeEngine;

  /// Action dispatch method name on the bindings object.
  final String dispatch;

  /// Snapshot stream method name on the bindings object.
  final String stateStream;

  /// Current snapshot method name on the bindings object.
  final String current;

  /// Optional one-time application initialization hook method name.
  ///
  /// When set, the generated controller calls this hook once as part of store
  /// initialization (before creating the engine). This can be used for
  /// per-library initialization steps required by your bindings layer.
  ///
  /// In FRB-based apps, Oxide recommends calling `RustLib.init()` and the
  /// app-provided `initOxide()` from `main()` instead of relying on this hook.
  final String? initApp;

  /// Optional method name for encoding current state bytes for persistence.
  final String? encodeCurrentState;

  /// Optional method name for encoding a provided state value.
  final String? encodeState;

  /// Optional method name for decoding bytes into a state value.
  final String? decodeState;

  /// Optional prefix override used for generated type names.
  final String? name;
}
