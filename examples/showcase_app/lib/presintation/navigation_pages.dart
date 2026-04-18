import 'package:flutter/widgets.dart';
import 'package:oxide_annotations/oxide_annotations.dart';
import 'package:showcase_app/oxide.dart';
import 'package:showcase_app/presintation/screens/home_screen.dart'
    as home_screen;
import 'package:showcase_app/presintation/screens/settings_screen.dart'
    as settings_screen;
import 'package:showcase_app/presintation/screens/splash_screen.dart'
    as splash_screen;

@OxideRoutePage(RouteKind.homeScreen)
final class HomePage extends StatelessWidget {
  const HomePage({super.key, required this.route});

  final HomeScreen route;

  @override
  Widget build(BuildContext context) {
    return const home_screen.HomeScreen();
  }
}

@OxideRoutePage(RouteKind.settingsScreen)
final class SettingsPage extends StatelessWidget {
  const SettingsPage({super.key, required this.route});

  final SettingsScreen route;

  @override
  Widget build(BuildContext context) {
    return const settings_screen.SettingsScreen();
  }
}

@OxideRoutePage(RouteKind.splashScreen)
final class SplashPage extends StatelessWidget {
  const SplashPage({super.key, required this.route});

  final SplashScreen route;

  @override
  Widget build(BuildContext context) {
    return const splash_screen.SplashScreen();
  }
}
