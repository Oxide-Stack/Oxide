import 'package:flutter/material.dart';

Color? parseHexColor(String? raw) {
  final value = raw?.trim();
  if (value == null || value.isEmpty) return null;

  final sanitized = value.replaceFirst('#', '');
  final normalized = switch (sanitized.length) {
    6 => 'ff$sanitized',
    8 => sanitized,
    _ => null,
  };
  if (normalized == null) return null;

  final parsed = int.tryParse(normalized, radix: 16);
  if (parsed == null) return null;
  return Color(parsed);
}

String colorToHex(Color color) =>
    '#${color.toARGB32().toRadixString(16).substring(2).toUpperCase()}';
