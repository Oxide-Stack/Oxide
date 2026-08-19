final class OxideStack {
  static bool _initialized = false;

  static bool get isInitialized => _initialized;

  static Future<void> init({bool startNavigation = true}) async {
    _initialized = true;
  }

  static OxideEvents get events => _events;
  static final _events = OxideEvents._();

  static OxideCallbacks get callbacks => _callbacks;
  static final _callbacks = OxideCallbacks._();
}

final class OxideEvents {
  const OxideEvents._();
}

final class OxideCallbacks {
  const OxideCallbacks._();
}
