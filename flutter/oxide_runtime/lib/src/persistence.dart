import 'dart:async';

/// Decodes a state value from persisted bytes.
typedef OxideDecodeState<S> = FutureOr<S> Function(List<int> bytes);

/// Byte storage abstraction used for persistence.
///
/// Implementations may use the local filesystem, shared preferences, secure
/// storage, or any other mechanism that can read and write raw bytes.
abstract interface class OxideByteStorage {
  /// Reads a previously stored payload for [key].
  ///
  /// # Returns
  /// The stored bytes, or `null` if the key does not exist.
  FutureOr<List<int>?> read(String key);

  /// Stores [bytes] under [key].
  ///
  /// Implementations may overwrite existing values.
  FutureOr<void> write(String key, List<int> bytes);
}
