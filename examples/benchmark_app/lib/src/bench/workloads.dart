import 'dart:collection';
import 'dart:convert';
import 'dart:math';

const fnvOffsetBasis = 0xcbf29ce484222325;
const fnvPrime = 0x100000001b3;
const u64Mask = 0xffffffffffffffff;

int fnv1a64Bytes(Iterable<int> bytes) {
  var hash = fnvOffsetBasis;
  for (final b in bytes) {
    hash ^= b;
    hash = (hash * fnvPrime) & u64Mask;
  }
  return hash;
}

int fnv1aMixU64(int hash, int v) {
  var h = hash;
  final x = v & u64Mask;
  for (var i = 0; i < 8; i++) {
    final b = (x >> (8 * i)) & 0xff;
    h ^= b;
    h = (h * fnvPrime) & u64Mask;
  }
  return h;
}

Object? canonicalizeJson(Object? value) {
  if (value is Map) {
    final out = SplayTreeMap<String, Object?>();
    for (final key in value.keys.whereType<String>()) {
      out[key] = canonicalizeJson(value[key]);
    }
    return out;
  }
  if (value is List) {
    return [for (final v in value) canonicalizeJson(v)];
  }
  return value;
}

int countEntries(Object? decoded) {
  if (decoded is! Map) return 0;
  final entries = decoded['entries'];
  if (entries is List) return entries.length;
  return 0;
}

const sieveLimit = 50000;

int runSieve(int limit) {
  final isPrime = List<bool>.filled(limit + 1, true);
  if (isPrime.isNotEmpty) isPrime[0] = false;
  if (isPrime.length > 1) isPrime[1] = false;

  final sqrtLimit = sqrt(limit.toDouble()).floor();
  for (var p = 2; p <= sqrtLimit; p++) {
    if (!isPrime[p]) continue;
    var multiple = p * p;
    while (multiple <= limit) {
      isPrime[multiple] = false;
      multiple += p;
    }
  }

  var primes = 0;
  for (final v in isPrime) {
    if (v) primes++;
  }
  return primes;
}

(int counter, int checksum) runCounterWorkload({required int counter, required int checksum, required int iterations}) {
  if (iterations <= 0) return (counter, checksum);
  var c = counter;
  var h = checksum;
  for (var i = 0; i < iterations; i++) {
    c++;
    h = fnv1aMixU64(h, c);
  }
  return (c, h);
}

(int counter, int checksum) runJsonWorkload({required int counter, required int checksum, required String json, required int iterations}) {
  if (iterations <= 0) return (counter, checksum);
  var c = counter;
  var h = checksum;
  for (var i = 0; i < iterations; i++) {
    final decoded = jsonDecode(json);
    final canonical = canonicalizeJson(decoded);
    final entries = countEntries(canonical);
    final serialized = jsonEncode(canonical);
    final hash = fnv1a64Bytes(utf8.encode(serialized));
    c++;
    h = fnv1aMixU64(h, hash);
    h = fnv1aMixU64(h, entries);
    h = fnv1aMixU64(h, serialized.length);
  }
  return (c, h);
}

(int counter, int checksum) runSieveWorkload({required int counter, required int checksum, required int iterations}) {
  if (iterations <= 0) return (counter, checksum);
  var c = counter;
  var h = checksum;
  for (var i = 0; i < iterations; i++) {
    final primes = runSieve(sieveLimit);
    c++;
    h = fnv1aMixU64(h, primes);
    h = fnv1aMixU64(h, sieveLimit);
  }
  return (c, h);
}
