import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:oxide_runtime/oxide_runtime.dart';

import '../../oxide.dart';
import '../../rust/state/app_state.dart';

final class TodosHomeScreen extends StatelessWidget {
  const TodosHomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 4,
      child: Scaffold(
        appBar: AppBar(
          title: const Text('Todos (4 Backends)'),
          actions: [
            Consumer(
              builder: (context, ref, _) {
                final actions = ref.read(todosListRiverpodOxideProvider).actions;
                return IconButton(
                  onPressed: () => actions.openConfirm(title: 'Confirm from Rust navigation?'),
                  icon: const Icon(Icons.help_outline),
                );
              },
            ),
          ],
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
            TodosListInheritedOxideScope(child: TodosNextIdInheritedOxideScope(child: _InheritedPane())),
            TodosListHooksOxideScope(child: TodosNextIdHooksOxideScope(child: _HooksPane())),
            _RiverpodPane(),
            _BlocPane(),
          ],
        ),
      ),
    );
  }
}

class _LoadingView extends StatelessWidget {
  const _LoadingView();

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Semantics(
        label: 'Loading',
        child: const CircularProgressIndicator(),
      ),
    );
  }
}

class _ErrorView extends StatelessWidget {
  const _ErrorView(this.error);

  final Object? error;

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('Something went wrong', style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 8),
            Text('$error', textAlign: TextAlign.center),
          ],
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
    final listController = TodosListInheritedOxideScope.controllerOf(context);
    final nextIdController = TodosNextIdInheritedOxideScope.controllerOf(context);
    return Column(
      children: [
        Expanded(
          child: AnimatedBuilder(
            animation: nextIdController,
            builder: (context, _) {
              final view = nextIdController.oxide;
              final state = view.state;
              if (view.isLoading) return const _LoadingView();
              if (view.error != null) return _ErrorView(view.error);
              if (state == null) return const Center(child: Text('No state'));
              return _NextIdSection(title: 'Next ID (slice: nextId)', nextId: state.nextId.toInt());
            },
          ),
        ),
        Expanded(
          flex: 3,
          child: AnimatedBuilder(
            animation: listController,
            builder: (context, _) {
              final view = listController.oxide;
              final state = view.state;
              if (view.isLoading) return const _LoadingView();
              if (view.error != null) return _ErrorView(view.error);
              if (state == null) return const Center(child: Text('No state'));
              return _TodosListSection(
                title: 'Todos (slice: todos)',
                todos: state.todos,
                controller: _todoTitleController,
                onAdd: (title) => view.actions.addTodo(title: title),
                onToggle: (id) => view.actions.toggleTodo(id: id),
                onDelete: (id) => view.actions.deleteTodo(id: id),
              );
            },
          ),
        ),
      ],
    );
  }
}

class _HooksPane extends HookWidget {
  const _HooksPane();

  @override
  Widget build(BuildContext context) {
    final titleController = useTextEditingController();
    final listView = useTodosListHooksOxideOxide();
    final nextIdView = useTodosNextIdHooksOxideOxide();
    return Column(
      children: [
        Expanded(
          child: Builder(
            builder: (context) {
              final state = nextIdView.state;
              if (nextIdView.isLoading) return const _LoadingView();
              if (nextIdView.error != null) return _ErrorView(nextIdView.error);
              if (state == null) return const Center(child: Text('No state'));
              return _NextIdSection(title: 'Next ID (slice: nextId)', nextId: state.nextId.toInt());
            },
          ),
        ),
        Expanded(
          flex: 3,
          child: Builder(
            builder: (context) {
              final state = listView.state;
              if (listView.isLoading) return const _LoadingView();
              if (listView.error != null) return _ErrorView(listView.error);
              if (state == null) return const Center(child: Text('No state'));
              return _TodosListSection(
                title: 'Todos (slice: todos)',
                todos: state.todos,
                controller: titleController,
                onAdd: (title) => listView.actions.addTodo(title: title),
                onToggle: (id) => listView.actions.toggleTodo(id: id),
                onDelete: (id) => listView.actions.deleteTodo(id: id),
              );
            },
          ),
        ),
      ],
    );
  }
}

class _RiverpodPane extends ConsumerWidget {
  const _RiverpodPane();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Column(
      children: const [
        Expanded(child: _RiverpodNextIdSection()),
        Expanded(flex: 3, child: _RiverpodTodosListSection()),
      ],
    );
  }
}

class _BlocPane extends StatelessWidget {
  const _BlocPane();

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider(create: (_) => TodosNextIdBlocOxideCubit()),
        BlocProvider(create: (_) => TodosListBlocOxideCubit()),
      ],
      child: Column(
        children: const [
          Expanded(child: _BlocNextIdSection()),
          Expanded(flex: 3, child: _BlocTodosListSection()),
        ],
      ),
    );
  }
}

class _NextIdSection extends StatelessWidget {
  const _NextIdSection({required this.title, required this.nextId});

  final String title;
  final int nextId;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(title, style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 12),
          Text('nextId: $nextId'),
        ],
      ),
    );
  }
}

class _TodosListSection extends StatelessWidget {
  const _TodosListSection({
    required this.title,
    required this.todos,
    required this.controller,
    required this.onAdd,
    required this.onToggle,
    required this.onDelete,
  });

  final String title;
  final List<TodoItem> todos;
  final TextEditingController controller;
  final Future<void> Function(String title) onAdd;
  final Future<void> Function(String id) onToggle;
  final Future<void> Function(String id) onDelete;

  @override
  Widget build(BuildContext context) {
    return _TodosListSectionBody(
      title: title,
      todos: todos,
      controller: controller,
      onAdd: onAdd,
      onToggle: onToggle,
      onDelete: onDelete,
    );
  }
}

class _TodosListSectionBody extends StatefulWidget {
  const _TodosListSectionBody({
    required this.title,
    required this.todos,
    required this.controller,
    required this.onAdd,
    required this.onToggle,
    required this.onDelete,
  });

  final String title;
  final List<TodoItem> todos;
  final TextEditingController controller;
  final Future<void> Function(String title) onAdd;
  final Future<void> Function(String id) onToggle;
  final Future<void> Function(String id) onDelete;

  @override
  State<_TodosListSectionBody> createState() => _TodosListSectionBodyState();
}

class _TodosListSectionBodyState extends State<_TodosListSectionBody> {
  bool _inFlight = false;

  Future<void> _run(Future<void> Function() op) async {
    if (_inFlight) return;
    setState(() => _inFlight = true);
    try {
      await op();
    } finally {
      if (mounted) setState(() => _inFlight = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final canAdd = widget.controller.text.trim().isNotEmpty;
    return Padding(
      padding: const EdgeInsets.all(16),
      child: ListView(
        children: [
          Text(widget.title, style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 12),
          Text('Todos: ${widget.todos.length}'),
          const SizedBox(height: 12),
          TextField(
            controller: widget.controller,
            onChanged: (_) => setState(() {}),
            decoration: const InputDecoration(
              labelText: 'Add Todo Title (must be non-empty)',
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 8),
          if (_inFlight) const LinearProgressIndicator(),
          if (_inFlight) const SizedBox(height: 8),
          FilledButton(
            onPressed: _inFlight || !canAdd
                ? null
                : () => _run(() async {
                    final title = widget.controller.text.trim();
                    await widget.onAdd(title);
                    widget.controller.clear();
                    if (mounted) setState(() {});
                  }),
            child: const Text('Add Todo'),
          ),
          const SizedBox(height: 16),
          const Text('Todos'),
          const SizedBox(height: 8),
          if (widget.todos.isEmpty)
            const Text('No todos yet')
          else
            ...widget.todos.map(
              (t) => Row(
                children: [
                  Expanded(
                    child: CheckboxListTile(
                      value: t.completed,
                      onChanged: _inFlight ? null : (_) => _run(() => widget.onToggle(t.id)),
                      title: Text(t.title),
                      controlAffinity: ListTileControlAffinity.leading,
                      contentPadding: EdgeInsets.zero,
                    ),
                  ),
                  IconButton(
                    onPressed: _inFlight ? null : () => _run(() => widget.onDelete(t.id)),
                    icon: const Icon(Icons.delete),
                  ),
                ],
              ),
            ),
        ],
      ),
    );
  }
}

class _RiverpodNextIdSection extends ConsumerWidget {
  const _RiverpodNextIdSection();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final view = ref.watch(todosNextIdRiverpodOxideProvider);
    final state = view.state;
    if (view.isLoading) return const _LoadingView();
    if (view.error != null) return _ErrorView(view.error);
    if (state == null) return const Center(child: Text('No state'));
    return _NextIdSection(title: 'Next ID (slice: nextId)', nextId: state.nextId.toInt());
  }
}

class _RiverpodTodosListSection extends ConsumerStatefulWidget {
  const _RiverpodTodosListSection();

  @override
  ConsumerState<_RiverpodTodosListSection> createState() => _RiverpodTodosListSectionState();
}

class _RiverpodTodosListSectionState extends ConsumerState<_RiverpodTodosListSection> {
  late final TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final view = ref.watch(todosListRiverpodOxideProvider);
    final state = view.state;
    if (view.isLoading) return const _LoadingView();
    if (view.error != null) return _ErrorView(view.error);
    if (state == null) return const Center(child: Text('No state'));
    final actions = ref.read(todosListRiverpodOxideProvider).actions;
    return _TodosListSection(
      title: 'Todos (slice: todos)',
      todos: state.todos,
      controller: _controller,
      onAdd: (title) => actions.addTodo(title: title),
      onToggle: (id) => actions.toggleTodo(id: id),
      onDelete: (id) => actions.deleteTodo(id: id),
    );
  }
}

class _BlocNextIdSection extends StatelessWidget {
  const _BlocNextIdSection();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<TodosNextIdBlocOxideCubit, OxideView<AppState, TodosNextIdBlocOxideActions>>(
      builder: (context, view) {
        final state = view.state;
        if (view.isLoading) return const _LoadingView();
        if (view.error != null) return _ErrorView(view.error);
        if (state == null) return const Center(child: Text('No state'));
        return _NextIdSection(title: 'Next ID (slice: nextId)', nextId: state.nextId.toInt());
      },
    );
  }
}

class _BlocTodosListSection extends StatefulWidget {
  const _BlocTodosListSection();

  @override
  State<_BlocTodosListSection> createState() => _BlocTodosListSectionState();
}

class _BlocTodosListSectionState extends State<_BlocTodosListSection> {
  late final TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<TodosListBlocOxideCubit, OxideView<AppState, TodosListBlocOxideActions>>(
      builder: (context, view) {
        final state = view.state;
        if (view.isLoading) return const _LoadingView();
        if (view.error != null) return _ErrorView(view.error);
        if (state == null) return const Center(child: Text('No state'));
        final actions = context.read<TodosListBlocOxideCubit>().actions;
        return _TodosListSection(
          title: 'Todos (slice: todos)',
          todos: state.todos,
          controller: _controller,
          onAdd: (title) => actions.addTodo(title: title),
          onToggle: (id) => actions.toggleTodo(id: id),
          onDelete: (id) => actions.deleteTodo(id: id),
        );
      },
    );
  }
}
