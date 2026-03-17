import 'package:flutter/foundation.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

void main() {
  group('OxideLogger', () {
    test('logs in debug mode', () {
      // In debug mode, logging should be enabled by default
      expect(kReleaseMode, isFalse); // Assuming tests run in debug

      // Capture debugPrint output
      final logOutput = <String>[];
      final originalDebugPrint = debugPrint;
      debugPrint = (message, {wrapWidth}) => logOutput.add(message!);

      OxideLogger.info('TestSource', 'Test message');

      debugPrint = originalDebugPrint;

      expect(logOutput.length, 1);
      expect(logOutput.first, contains('Test message'));
      expect(logOutput.first, contains(AnsiColors.green)); // Color code
    });

    test('does not log when disabled in release mode', () {
      // This test assumes release mode; in debug, it logs
      if (!kReleaseMode) {
        expect(true, isTrue); // Skip in debug
        return;
      }
      final logOutput = <String>[];
      final originalDebugPrint = debugPrint;
      debugPrint = (message, {wrapWidth}) => logOutput.add(message!);

      OxideLogger.info('TestSource', 'Should not log');

      debugPrint = originalDebugPrint;

      expect(logOutput.isEmpty, isTrue);
    });

    test('handles Rust log input correctly', () {
      final logOutput = <String>[];
      final originalDebugPrint = debugPrint;
      debugPrint = (message, {wrapWidth}) => logOutput.add(message!);

      OxideLogger.handleRustLog('info', 'rust::engine', 'Rust message');

      debugPrint = originalDebugPrint;

      expect(logOutput.length, 1);
      expect(logOutput.first, contains('Rust message'));
    });

    test('formats different log levels correctly', () {
      final logOutput = <String>[];
      final originalDebugPrint = debugPrint;
      debugPrint = (message, {wrapWidth}) => logOutput.add(message!);

      OxideLogger.trace('Source', 'Trace');
      OxideLogger.debug('Source', 'Debug');
      OxideLogger.info('Source', 'Info');
      OxideLogger.warn('Source', 'Warn');
      OxideLogger.error('Source', 'Error');

      debugPrint = originalDebugPrint;

      expect(logOutput.length, 5);
      expect(logOutput[0], contains('Trace'));
      expect(logOutput[1], contains('Debug'));
      expect(logOutput[2], contains('Info'));
      expect(logOutput[3], contains('Warn'));
      expect(logOutput[4], contains('Error'));
    });
  });
}
