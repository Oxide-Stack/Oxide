import 'package:flutter/material.dart';

final class BenchDetailScreen extends StatelessWidget {
  const BenchDetailScreen({super.key, required this.id});

  final int id;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Benchmark $id'),
        leading: IconButton(
          icon: const Icon(Icons.arrow_back),
          onPressed: () => Navigator.of(context).maybePop(),
        ),
      ),
      body: Center(child: Text('Deep link param id=$id')),
    );
  }
}

