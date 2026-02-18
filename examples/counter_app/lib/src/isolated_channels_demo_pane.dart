import 'dart:async';

import 'package:flutter/material.dart';

import 'rust/api/isolated_channels_bridge.dart' as ch;
import 'rust/isolated_channels_demo/channels.dart';

/// Demonstrates Oxide isolated channels by exercising the FRB bridge from UI.
///
/// This pane is intentionally interactive (not just a styling demo):
/// - starts the Rust isolated-channels demo runtime
/// - listens to Rust → Dart event streams
/// - handles Rust → Dart → Rust callback requests via a real dialog + respond()
/// - sends duplex messages both directions and renders what is observed
final class IsolatedChannelsDemoPane extends StatefulWidget {
  const IsolatedChannelsDemoPane({super.key});

  @override
  State<IsolatedChannelsDemoPane> createState() => _IsolatedChannelsDemoPaneState();
}

final class _IsolatedChannelsDemoPaneState extends State<IsolatedChannelsDemoPane> {
  final List<String> _log = <String>[];
  final TextEditingController _messageController = TextEditingController(text: 'Hello from Flutter');
  final TextEditingController _confirmTitleController = TextEditingController(text: 'Do you confirm?');

  StreamSubscription<CounterDemoEvent>? _eventsSub;
  StreamSubscription<CounterDemoOut>? _duplexOutSub;
  StreamSubscription<ch.CounterDemoDialogPendingRequest>? _dialogReqSub;

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
      await ch.initIsolatedChannelsDemo();

      _eventsSub = ch.counterDemoEventsStream().listen((event) {
        event.when(
          notify: (message) => _append('event.notify: $message'),
        );
      });

      _duplexOutSub = ch.counterDemoDuplexOutgoingStream().listen((event) {
        event.when(send: (text) => _append('duplex.out: $text'));
      });

      _dialogReqSub = ch.counterDemoDialogRequestsStream().listen((pending) {
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

  Future<void> _handleDialogRequest(ch.CounterDemoDialogPendingRequest pending) async {
    final request = pending.request;
    final response = await request.when<Future<CounterDemoDialogResponse>>(
      confirm: (title) async {
        final result = await showDialog<bool>(
          context: context,
          builder: (context) {
            return AlertDialog(
              title: const Text('Rust → Dart callback'),
              content: Text(title),
              actions: [
                TextButton(onPressed: () => Navigator.of(context).pop(false), child: const Text('Cancel')),
                FilledButton(onPressed: () => Navigator.of(context).pop(true), child: const Text('Confirm')),
              ],
            );
          },
        );
        _append('callback.confirm answered: ${result ?? false}');
        return CounterDemoDialogResponse.confirm(result ?? false);
      },
    );

    await ch.counterDemoDialogRespond(id: pending.id, response: response);
  }

  void _append(String line) {
    if (!mounted) return;
    setState(() {
      _log.insert(0, line);
      if (_log.length > 200) _log.removeRange(200, _log.length);
    });
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
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
                        await ch.emitCounterDemoNotification(message: message.isEmpty ? 'Hello' : message);
                      },
                child: const Text('Emit Event'),
              ),
              FilledButton(
                onPressed: !_started
                    ? null
                    : () async {
                        final title = _confirmTitleController.text.trim();
                        final ok = await ch.counterDemoDialogConfirm(title: title.isEmpty ? 'Confirm?' : title);
                        if (!context.mounted) return;
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
                        final title = _confirmTitleController.text.trim();
                        final ok = await ch.counterDemoDialogConfirmViaFrbCallback(
                          title: title.isEmpty ? 'Confirm?' : title,
                          dartConfirm: (t) async {
                            final result = await showDialog<bool>(
                              context: context,
                              builder: (context) {
                                return AlertDialog(
                                  title: const Text('Direct FRB callback'),
                                  content: Text(t),
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
                            _append('frb.callback answered: ${result ?? false}');
                            return result ?? false;
                          },
                        );
                        if (!context.mounted) return;
                        ScaffoldMessenger.of(context).showSnackBar(
                          SnackBar(content: Text('Direct FRB callback result: $ok')),
                        );
                      },
                child: const Text('Direct FRB Callback'),
              ),
              FilledButton(
                onPressed: !_started
                    ? null
                    : () async {
                        final message = _messageController.text.trim();
                        await ch.counterDemoDuplexSend(text: message.isEmpty ? 'Hello' : message);
                      },
                child: const Text('Send Duplex Out'),
              ),
              FilledButton(
                onPressed: !_started
                    ? null
                    : () async {
                        final message = _messageController.text.trim();
                        await ch.counterDemoDuplexIncoming(event: CounterDemoIn.receive(text: message.isEmpty ? 'Hello' : message));
                        final last = await ch.counterDemoLastIncomingText();
                        _append('duplex.in stored in Rust: ${last ?? "null"}');
                      },
                child: const Text('Send Duplex In'),
              ),
            ],
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _messageController,
            decoration: const InputDecoration(labelText: 'Message', border: OutlineInputBorder()),
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _confirmTitleController,
            decoration: const InputDecoration(labelText: 'Confirm title', border: OutlineInputBorder()),
          ),
          if (_startError != null) ...[
            const SizedBox(height: 12),
            Text('Start error: $_startError', style: TextStyle(color: Theme.of(context).colorScheme.error)),
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
                itemBuilder: (context, index) => ListTile(dense: true, title: Text(_log[index])),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
