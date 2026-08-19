#[tokio::test]
async fn callback_runtime_resolves_pending_requests() {
    crate::init_isolated_channels().unwrap();

    let runtime = std::sync::Arc::new(crate::CallbackRuntime::<u32, u32>::new(8));
    let runtime_responder = runtime.clone();

    let responder = tokio::spawn(async move {
        let (id, req) = runtime_responder.recv_request().await.expect("request");
        runtime_responder
            .respond(id, req + 1)
            .await
            .expect("respond");
    });

    let out = runtime.invoke(41).await.expect("invoke");
    assert_eq!(out, 42);
    responder.await.unwrap();
}

#[tokio::test]
async fn callback_runtime_rejects_unknown_response_ids() {
    crate::init_isolated_channels().unwrap();

    let runtime = crate::CallbackRuntime::<u32, u32>::new(8);
    let err = runtime.respond(999, 1).await.unwrap_err();
    assert_eq!(err, crate::OxideChannelError::UnexpectedResponse);
}

#[tokio::test]
async fn callback_runtime_respond_reports_unavailable_when_waiter_dropped() {
    crate::init_isolated_channels().unwrap();

    let runtime = std::sync::Arc::new(crate::CallbackRuntime::<u32, u32>::new(8));
    let runtime_invoke = runtime.clone();

    let invoke_task = tokio::spawn(async move { runtime_invoke.invoke(41).await });

    let (id, _req) = runtime.recv_request().await.expect("request");
    invoke_task.abort();
    let _ = invoke_task.await;

    let err = runtime.respond(id, 42).await.unwrap_err();
    assert_eq!(err, crate::OxideChannelError::Unavailable);
}

#[tokio::test]
async fn callback_runtime_recv_request_returns_none_when_not_initialized() {
    let runtime = crate::CallbackRuntime::<u32, u32>::new(8);
    let received =
        tokio::time::timeout(std::time::Duration::from_millis(30), runtime.recv_request()).await;
    assert!(received.is_err() || received.unwrap().is_none());
}

#[tokio::test]
async fn event_channel_delivers_events_to_subscribers() {
    crate::init_isolated_channels().unwrap();

    let runtime = crate::EventChannelRuntime::<u32>::new(8);
    let mut rx = runtime.subscribe();
    runtime.emit(7);

    let next = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
        .await
        .expect("recv")
        .expect("event");
    assert_eq!(next, 7);
}

#[tokio::test]
async fn incoming_handler_reports_unavailable_when_missing() {
    crate::init_isolated_channels().unwrap();

    let handler = crate::IncomingHandler::<u32>::new();
    let err = handler.handle(1).unwrap_err();
    assert_eq!(err, crate::OxideChannelError::Unavailable);
}

#[tokio::test]
async fn incoming_handler_converts_panics_to_platform_error() {
    crate::init_isolated_channels().unwrap();

    let handler = crate::IncomingHandler::<u32>::new();
    handler.register(|_| panic!("boom"));

    let err = handler.handle(1).unwrap_err();
    assert!(matches!(err, crate::OxideChannelError::PlatformError(_)));
}

#[tokio::test]
async fn incoming_handler_invokes_registered_handler() {
    crate::init_isolated_channels().unwrap();

    let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u32>::new()));
    let seen_clone = seen.clone();

    let handler = crate::IncomingHandler::<u32>::new();
    handler.register(move |value| {
        seen_clone.lock().unwrap().push(value);
    });

    handler.handle(7).unwrap();
    let values = seen.lock().unwrap().clone();
    assert_eq!(values, vec![7]);
}
