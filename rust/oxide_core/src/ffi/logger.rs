use std::sync::OnceLock;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::{filter::LevelFilter, Layer};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::prelude::*;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender, UnboundedReceiver};

/// The log levels exposed to Dart as strings
fn map_level(level: tracing::Level) -> &'static str {
    match level {
        tracing::Level::TRACE => "trace",
        tracing::Level::DEBUG => "debug",
        tracing::Level::INFO => "info",
        tracing::Level::WARN => "warn",
        tracing::Level::ERROR => "error",
    }
}

static LOG_SINK: OnceLock<UnboundedSender<(String, String, String)>> = OnceLock::new();

/// A custom tracing layer that forwards events to an mpsc sender.
pub struct FrbLogLayer;

impl<S> Layer<S> for FrbLogLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        if let Some(sink) = LOG_SINK.get() {
            let mut visitor = StringVisitor::new();
            event.record(&mut visitor);

            let entry_level = map_level(*event.metadata().level()).to_string();
            let entry_target = event.metadata().target().to_string();
            let entry_message = visitor.message;

            // ignore disconnected errors
            let _ = sink.send((entry_level, entry_target, entry_message));
        }
    }
}

struct StringVisitor {
    message: String,
}

impl StringVisitor {
    fn new() -> Self {
        Self {
            message: String::new(),
        }
    }
}

impl tracing::field::Visit for StringVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }
}

/// Initializes tracing and returns a receiver for the log events.
pub fn setup_log_stream() -> Option<UnboundedReceiver<(String, String, String)>> {
    let (tx, rx) = unbounded_channel();

    if LOG_SINK.set(tx).is_err() {
        return None; // Already initialized
    }

    // Set up tracing subscriber
    let _ = tracing_subscriber::registry()
        .with(LevelFilter::TRACE)
        .with(FrbLogLayer)
        .try_init();

    Some(rx)
}
