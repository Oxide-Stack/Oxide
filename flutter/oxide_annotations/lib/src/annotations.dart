/// Backend integration strategy used by generated store code.
enum OxideBackend { inherited, inheritedHooks, riverpod, bloc }

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
    this.backend = OxideBackend.inherited,
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
  /// Which Flutter state-management backend to generate for.
  final OxideBackend backend;

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
