import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';
import 'package:showcase_app/oxide.dart';
import 'package:showcase_app/presintation/color_utils.dart';
import 'package:showcase_app/presintation/navigation_pages.dart';
import 'package:showcase_app/presintation/controllers/settings_controller.dart';
import 'package:showcase_app/src/rust/config/enums/theme_type.dart'
    as rust_theme;

@OxideApp(navigation: OxideNavigation.navigator())
class MyApp extends ConsumerWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final settings = ref.watch(settingsControllerProvider);
    final state = settings.state;
    final seedColor = parseHexColor(state?.mainColor) ?? Colors.blue;
    final isDark = state?.theme == rust_theme.Theme.dark;

    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Oxide Showcase',
      navigatorKey: OxideStack.navigatorKey,
      themeMode: isDark ? ThemeMode.dark : ThemeMode.light,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: seedColor,
          brightness: Brightness.light,
        ),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: seedColor,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      home: const SplashPage(route: SplashScreen()),
    );
  }
}
