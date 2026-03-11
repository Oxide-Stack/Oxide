// Transitional shim: many examples imported the older
// `navigation_bridge.dart` for convenience.  The modern API has been
// unified under `routes/oxide_navigation.dart` and the bridge methods
// exposed there; this file simply re-exports that surface so existing
// imports continue to work while keeping the definitions in one place.

export '../routes/oxide_navigation.dart';
