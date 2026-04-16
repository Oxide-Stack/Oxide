import 'package:flutter/material.dart';
import 'package:showcase_app/oxide_generated/oxide_stack.g.dart'
    show runOxideApp;
import 'package:showcase_app/src/rust/frb_generated.dart';

Future<void> main() async {
  runOxideApp(const MyApp());
}
