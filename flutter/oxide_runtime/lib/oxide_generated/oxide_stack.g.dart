final class OxideStack {
  static bool _initialized = false;

  static bool get isInitialized => _initialized;

  static Future<void> init({bool startNavigation = true}) async {
    _initialized = true;
  }

  static _OxideEvents get events => _oxideEvents;
  static final _oxideEvents = _OxideEvents._();

  static _OxideCallbacks get callbacks => _oxideCallbacks;
  static final _oxideCallbacks = _OxideCallbacks._();
}

class _OxideEvents {
  const _OxideEvents._();
}

class _OxideCallbacks {
  const _OxideCallbacks._();
}
