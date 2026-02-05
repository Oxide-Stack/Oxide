import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import 'src/oxide.dart';
import 'src/rust/api/bridge.dart' show initOxide;
import 'src/rust/frb_generated.dart';
import 'src/rust/state/app_state.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init();
  await initOxide();
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: DefaultTabController(
        length: 4,
        child: Scaffold(
          appBar: AppBar(
            title: const Text('Todos (4 Backends)'),
            bottom: const TabBar(
              tabs: [
                Tab(text: 'Inherited'),
                Tab(text: 'Hooks'),
                Tab(text: 'Riverpod'),
                Tab(text: 'BLoC'),
              ],
            ),
          ),
          body: const TabBarView(
            children: [
              StateBridgeOxideScope(child: _InheritedPane()),
              StateBridgeHooksOxideScope(child: _HooksPane()),
              _RiverpodPane(),
              _BlocPane(),
            ],
          ),
        ),
      ),
    );
  }
}

class _InheritedPane extends StatefulWidget {
  const _InheritedPane();

  @override
  State<_InheritedPane> createState() => _InheritedPaneState();
}

class _InheritedPaneState extends State<_InheritedPane> {
  final _todoTitleController = TextEditingController();

  @override
  void dispose() {
    _todoTitleController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final controller = StateBridgeOxideScope.controllerOf(context);
    return AnimatedBuilder(
      animation: controller,
      builder: (context, _) {
        final oxide = controller.oxide;
        final state = oxide.state;

        if (oxide.isLoading) {
          return const Center(child: Text('Loading state stream...'));
        }

        if (oxide.error != null) {
          return Center(
            child: Text('Error: ${oxide.error}', style: const TextStyle(color: Colors.red)),
          );
        }

        if (state == null) {
          return const Center(child: Text('No state available'));
        }

        return Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text('Todos: ${state.todos.length}'),
              const SizedBox(height: 16),
              Expanded(
                child: ListView(
                  children: [
                    TextField(
                      controller: _todoTitleController,
                      decoration: const InputDecoration(labelText: 'Add Todo Title (must be non-empty)', border: OutlineInputBorder()),
                    ),
                    const SizedBox(height: 8),
                    FilledButton(
                      onPressed: () {
                        // This dispatches to Rust; validation errors are reported via `oxide.error`.
                        oxide.actions.addTodo(title: _todoTitleController.text);
                        _todoTitleController.clear();
                      },
                      child: const Text('Add Todo'),
                    ),
                    const SizedBox(height: 16),
                    const Text('Todos'),
                    const SizedBox(height: 8),
                    if (state.todos.isEmpty)
                      const Text('No todos yet')
                    else
                      ...state.todos.map(
                        (t) => Row(
                          children: [
                            Expanded(
                              child: CheckboxListTile(
                                value: t.completed,
                                onChanged: (_) => oxide.actions.toggleTodo(id: t.id),
                                title: Text(t.title),
                                controlAffinity: ListTileControlAffinity.leading,
                                contentPadding: EdgeInsets.zero,
                              ),
                            ),
                            IconButton(
                              onPressed: () => oxide.actions.deleteTodo(id: t.id),
                              icon: const Icon(Icons.delete),
                            ),
                          ],
                        ),
                      ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

class _HooksPane extends HookWidget {
  const _HooksPane();

  @override
  Widget build(BuildContext context) {
    final view = useStateBridgeHooksOxideOxide();
    final state = view.state;
    final titleController = useTextEditingController();
    if (view.isLoading) return const Center(child: Text('Loading…'));
    if (view.error != null) return Center(child: Text('Error: ${view.error}'));
    if (state == null) return const Center(child: Text('No state'));
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Todos: ${state.todos.length}'),
          const SizedBox(height: 16),
          Expanded(
            child: ListView(
              children: [
                TextField(
                  controller: titleController,
                  decoration: const InputDecoration(labelText: 'Add Todo Title (must be non-empty)', border: OutlineInputBorder()),
                ),
                const SizedBox(height: 8),
                FilledButton(
                  onPressed: () {
                    view.actions.addTodo(title: titleController.text);
                    titleController.clear();
                  },
                  child: const Text('Add Todo'),
                ),
                const SizedBox(height: 16),
                const Text('Todos'),
                const SizedBox(height: 8),
                if (state.todos.isEmpty)
                  const Text('No todos yet')
                else
                  ...state.todos.map(
                    (t) => Row(
                      children: [
                        Expanded(
                          child: CheckboxListTile(
                            value: t.completed,
                            onChanged: (_) => view.actions.toggleTodo(id: t.id),
                            title: Text(t.title),
                            controlAffinity: ListTileControlAffinity.leading,
                            contentPadding: EdgeInsets.zero,
                          ),
                        ),
                        IconButton(
                          onPressed: () => view.actions.deleteTodo(id: t.id),
                          icon: const Icon(Icons.delete),
                        ),
                      ],
                    ),
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _RiverpodPane extends ConsumerWidget {
  const _RiverpodPane();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(stateBridgeRiverpodOxideProvider);
    final state = view.state;
    final actions = ref.read(stateBridgeRiverpodOxideProvider).actions;
    final titleController = TextEditingController();
    if (view.isLoading) return const Center(child: Text('Loading…'));
    if (view.error != null) return Center(child: Text('Error: ${view.error}'));
    if (state == null) return const Center(child: Text('No state'));
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('Todos: ${state.todos.length}'),
          const SizedBox(height: 16),
          Expanded(
            child: ListView(
              children: [
                TextField(
                  controller: titleController,
                  decoration: const InputDecoration(labelText: 'Add Todo Title (must be non-empty)', border: OutlineInputBorder()),
                ),
                const SizedBox(height: 8),
                FilledButton(
                  onPressed: () {
                    actions.addTodo(title: titleController.text);
                    titleController.clear();
                  },
                  child: const Text('Add Todo'),
                ),
                const SizedBox(height: 16),
                const Text('Todos'),
                const SizedBox(height: 8),
                if (state.todos.isEmpty)
                  const Text('No todos yet')
                else
                  ...state.todos.map(
                    (t) => Row(
                      children: [
                        Expanded(
                          child: CheckboxListTile(
                            value: t.completed,
                            onChanged: (_) => actions.toggleTodo(id: t.id),
                            title: Text(t.title),
                            controlAffinity: ListTileControlAffinity.leading,
                            contentPadding: EdgeInsets.zero,
                          ),
                        ),
                        IconButton(
                          onPressed: () => actions.deleteTodo(id: t.id),
                          icon: const Icon(Icons.delete),
                        ),
                      ],
                    ),
                  ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _BlocPane extends StatelessWidget {
  const _BlocPane();

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (_) => StateBridgeBlocOxideCubit(),
      child: BlocBuilder<StateBridgeBlocOxideCubit, OxideView<AppState, StateBridgeBlocOxideActions>>(
        builder: (context, view) {
          final state = view.state;
          final actions = context.read<StateBridgeBlocOxideCubit>().actions;
          final titleController = TextEditingController();
          if (view.isLoading) return const Center(child: Text('Loading…'));
          if (view.error != null) return Center(child: Text('Error: ${view.error}'));
          if (state == null) return const Center(child: Text('No state'));
          return Padding(
            padding: const EdgeInsets.all(16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('Todos: ${state.todos.length}'),
                const SizedBox(height: 16),
                Expanded(
                  child: ListView(
                    children: [
                      TextField(
                        controller: titleController,
                        decoration: const InputDecoration(labelText: 'Add Todo Title (must be non-empty)', border: OutlineInputBorder()),
                      ),
                      const SizedBox(height: 8),
                      FilledButton(
                        onPressed: () {
                          actions.addTodo(title: titleController.text);
                          titleController.clear();
                        },
                        child: const Text('Add Todo'),
                      ),
                      const SizedBox(height: 16),
                      const Text('Todos'),
                      const SizedBox(height: 8),
                      if (state.todos.isEmpty)
                        const Text('No todos yet')
                      else
                        ...state.todos.map(
                          (t) => Row(
                            children: [
                              Expanded(
                                child: CheckboxListTile(
                                  value: t.completed,
                                  onChanged: (_) => actions.toggleTodo(id: t.id),
                                  title: Text(t.title),
                                  controlAffinity: ListTileControlAffinity.leading,
                                  contentPadding: EdgeInsets.zero,
                                ),
                              ),
                              IconButton(
                                onPressed: () => actions.deleteTodo(id: t.id),
                                icon: const Icon(Icons.delete),
                              ),
                            ],
                          ),
                        ),
                    ],
                  ),
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}
