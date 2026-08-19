import 'dart:async';

import 'package:flutter/material.dart';
import 'package:showcase_app/src/rust/api/isolated_channels_bridge.dart';
import 'package:showcase_app/src/rust/isolated_channels_demo/channels.dart';

class IsolatedChannelsDemoPane extends StatefulWidget {
  const IsolatedChannelsDemoPane({super.key});

  @override
  State<IsolatedChannelsDemoPane> createState() => _IsolatedChannelsDemoPaneState();
}

class _IsolatedChannelsDemoPaneState extends State<IsolatedChannelsDemoPane> {
  final List<String> _log = <String>[];
  final TextEditingController _messageController = TextEditingController(
    text: 'Hello from Flutter',
  );
  final TextEditingController _confirmTitleController = TextEditingController(
    text: 'Do you confirm?',
  );

  StreamSubscription<ShowcaseDemoEvent>? _eventsSub;
  StreamSubscription<ShowcaseDemoOut>? _duplexOutSub;
  StreamSubscription<ShowcaseDemoDialogPendingRequest>? _dialogReqSub;

  bool _started = false;
  bool _starting = false;
  Object? _startError;

  @override
  void dispose() {
    _eventsSub?.cancel();
    _duplexOutSub?.cancel();
    _dialogReqSub?.cancel();
    _messageController.dispose();
    _confirmTitleController.dispose();
    super.dispose();
  }

  Future<void> _start() async {
    if (_started || _starting) return;
    setState(() {
      _starting = true;
      _startError = null;
    });

    try {
      await initIsolatedChannelsDemo();

      _eventsSub = showcaseDemoEventsStream().listen((event) {
        event.when(notify: (message) => _append('event.notify: $message'));
      });

      _duplexOutSub = showcaseDemoDuplexOutgoingStream().listen((event) {
        event.when(send: (text) => _append('duplex.out: $text'));
      });

      _dialogReqSub = showcaseDemoDialogRequestsStream().listen((pending) {
        unawaited(_handleDialogRequest(pending));
      });

      setState(() {
        _started = true;
      });
      _append('demo started');
    } catch (e) {
      setState(() {
        _startError = e;
      });
    } finally {
      if (mounted) {
        setState(() {
          _starting = false;
        });
      }
    }
  }

  Future<void> _handleDialogRequest(ShowcaseDemoDialogPendingRequest pending) async {
    final req = pending.request;
    if (req is! ShowcaseDemoDialogRequest_Confirm) {
      _append('unexpected callback request: $req');
      return;
    }
    final result = await showDialog<bool>(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: const Text('Rust → Dart callback'),
          content: Text(req.title),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(false),
              child: const Text('Cancel'),
            ),
            FilledButton(
              onPressed: () => Navigator.of(context).pop(true),
              child: const Text('Confirm'),
            ),
          ],
        );
      },
    );
    final response = ShowcaseDemoDialogResponse.confirm(result ?? false);
    await showcaseDemoDialogRespond(id: pending.id, response: response);
    _append('callback.confirm answered: ${result ?? false}');
  }

  void _append(String line) {
    if (!mounted) return;
    setState(() {
      _log.insert(0, line);
      if (_log.length > 100) {
        _log.removeRange(100, _log.length);
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: [
            FilledButton(
              onPressed: _starting ? null : _start,
              child: Text(_started ? 'Started' : (_starting ? 'Starting…' : 'Start Demo')),
            ),
            FilledButton(
              onPressed: !_started
                  ? null
                  : () async {
                      final message = _messageController.text.trim();
                      await emitShowcaseDemoNotification(
                        message: message.isEmpty ? 'Hello' : message,
                      );
                    },
              child: const Text('Emit Event'),
            ),
            FilledButton(
              onPressed: !_started
                  ? null
                  : () async {
                      final title = _confirmTitleController.text.trim();
                      final ok = await showcaseDemoDialogConfirm(
                        title: title.isEmpty ? 'Confirm?' : title,
                      );
                      if (!mounted) return;
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(content: Text('Rust confirm result: $ok')),
                      );
                    },
              child: const Text('Request Confirm'),
            ),
            FilledButton(
              onPressed: !_started
                  ? null
                  : () async {
                      final message = _messageController.text.trim();
                      await showcaseDemoDuplexSend(
                        text: message.isEmpty ? 'Hello' : message,
                      );
                    },
              child: const Text('Send Duplex Out'),
            ),
            FilledButton(
              onPressed: !_started
                  ? null
                  : () async {
                      final message = _messageController.text.trim();
                      await showcaseDemoDuplexIncoming(
                        event: ShowcaseDemoIn.receive(
                          text: message.isEmpty ? 'Hello' : message,
                        ),
                      );
                      final last = await showcaseDemoLastIncomingText();
                      _append('duplex.in stored in Rust: ${last ?? "null"}');
                    },
              child: const Text('Send Duplex In'),
            ),
          ],
        ),
        const SizedBox(height: 12),
        TextField(
          controller: _messageController,
          decoration: const InputDecoration(
            labelText: 'Message',
            border: OutlineInputBorder(),
          ),
        ),
        const SizedBox(height: 12),
        TextField(
          controller: _confirmTitleController,
          decoration: const InputDecoration(
            labelText: 'Confirm title',
            border: OutlineInputBorder(),
          ),
        ),
        if (_startError != null) ...[
          const SizedBox(height: 12),
          Text(
            'Start error: $_startError',
            style: TextStyle(color: Theme.of(context).colorScheme.error),
          ),
        ],
        const SizedBox(height: 12),
        Text('Log', style: Theme.of(context).textTheme.titleMedium),
        const SizedBox(height: 8),
        Expanded(
          child: DecoratedBox(
            decoration: BoxDecoration(
              border: Border.all(color: Theme.of(context).dividerColor),
              borderRadius: BorderRadius.circular(8),
            ),
            child: ListView.builder(
              itemCount: _log.length,
              itemBuilder: (context, index) => ListTile(
                dense: true,
                title: Text(_log[index]),
              ),
            ),
          ),
        ),
      ],
    );
  }
}
