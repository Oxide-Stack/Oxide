import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:showcase_app/presintation/controllers/settings_controller.dart';

class SplashScreen extends HookConsumerWidget {
  const SplashScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // Setup
    final controller = useAnimationController(
      duration: const Duration(milliseconds: 1200),
    )..repeat(reverse: true);
    final scaleAnimation = useAnimation<double>(
      Tween<double>(
        begin: 0.9,
        end: 1.1,
      ).animate(CurvedAnimation(parent: controller, curve: Curves.easeInOut)),
    );
    final opacityAnimation = useAnimation<double>(
      Tween<double>(
        begin: 0.6,
        end: 1.0,
      ).animate(CurvedAnimation(parent: controller, curve: Curves.easeInOut)),
    );
    useEffect(() {
      final timer = Timer(const Duration(seconds: 2), () {
        if (!context.mounted) return;
        ref.read(settingsControllerProvider).actions.openHome();
      });
      return timer.cancel;
    }, []);

    // Build Start
    return Scaffold(
      body: Container(
        width: double.infinity,
        height: double.infinity,
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [Colors.blue, Colors.indigo],
          ),
        ),
        child: Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              AnimatedBuilder(
                animation: controller,
                builder: (context, child) {
                  return Transform.scale(
                    scale: scaleAnimation,
                    child: Opacity(opacity: opacityAnimation, child: child),
                  );
                },
                child: const FlutterLogo(size: 96),
              ),
              const SizedBox(height: 24),
              const SizedBox(
                width: 28,
                height: 28,
                child: CircularProgressIndicator(
                  strokeWidth: 3,
                  valueColor: AlwaysStoppedAnimation<Color>(Colors.white),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
