import 'dart:io';

import 'package:benchmark_app/src/bench/workloads.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('Dart workloads match Rust parity constants', () {
    final lightJson = File('assets/light.json').readAsStringSync();
    final heavyJson = File('assets/heavy.json').readAsStringSync();

    expect(sieveLimit, 50000);
    expect(runSieve(sieveLimit), 5133);

    final (counterCounter, counterChecksum) = runCounterWorkload(counter: 0, checksum: fnvOffsetBasis, iterations: 1000);
    expect(counterCounter, 1000);
    expect(counterChecksum, 0x01f4b8025ff13a2c);

    final (sieveCounter, sieveChecksum) = runSieveWorkload(counter: 0, checksum: fnvOffsetBasis, iterations: 1);
    expect(sieveCounter, 1);
    expect(sieveChecksum, 0x61712022e7704885);

    final (lightCounter, lightChecksum) = runJsonWorkload(counter: 0, checksum: fnvOffsetBasis, json: lightJson, iterations: 1);
    expect(lightCounter, 1);
    expect(lightChecksum, 0x1b107b9bc4545255);

    final (heavyCounter, heavyChecksum) = runJsonWorkload(counter: 0, checksum: fnvOffsetBasis, json: heavyJson, iterations: 1);
    expect(heavyCounter, 1);
    expect(heavyChecksum, 0xb067dca7d88da466);
  });
}
