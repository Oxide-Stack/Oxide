import 'package:flutter/material.dart';
import 'package:oxide_runtime/oxide_runtime.dart';
import 'package:showcase_app/src/rust/routes.dart';

@OxideRoutePage(RouteKind.homeScreen)
class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(body: Center(child: Text('Home Screen')));
  }
}
