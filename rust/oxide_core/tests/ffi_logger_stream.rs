use tracing::{debug, error, info, trace, warn};

#[test]
fn log_stream_captures_all_levels_and_initializes_once() {
    let mut rx =
        oxide_core::ffi::logger::setup_log_stream().expect("first setup should initialize sink");

    trace!(target: "oxide_core::tests", "trace message");
    debug!(target: "oxide_core::tests", "debug message");
    info!(target: "oxide_core::tests", "info message");
    warn!(target: "oxide_core::tests", "warn message");
    error!(target: "oxide_core::tests", "error message");

    let mut entries = Vec::new();
    for _ in 0..5 {
        let entry = rx.blocking_recv().expect("expected forwarded log event");
        entries.push(entry);
    }

    assert!(entries.iter().any(|(level, target, message)| {
        level == "trace"
            && target.contains("oxide_core::tests")
            && message.contains("trace message")
    }));
    assert!(
        entries
            .iter()
            .any(|(level, _, message)| { level == "debug" && message.contains("debug message") })
    );
    assert!(
        entries
            .iter()
            .any(|(level, _, message)| { level == "info" && message.contains("info message") })
    );
    assert!(
        entries
            .iter()
            .any(|(level, _, message)| { level == "warn" && message.contains("warn message") })
    );
    assert!(
        entries
            .iter()
            .any(|(level, _, message)| { level == "error" && message.contains("error message") })
    );

    assert!(oxide_core::ffi::logger::setup_log_stream().is_none());
}
