#![cfg(feature = "navigation-binding")]

use oxide_core::navigation::{NavCommand, NavRoute};
use tokio_stream::StreamExt;

#[tokio::test]
async fn push_emits_a_command() {
    oxide_core::init_navigation().unwrap();
    let runtime = oxide_core::navigation_runtime().unwrap();

    let mut stream =
        tokio_stream::wrappers::BroadcastStream::new(runtime.subscribe_commands()).filter_map(|r| r.ok());

    runtime.push(NavRoute {
        kind: "Home".into(),
        payload: serde_json::json!({}),
        extras: None,
    });

    while let Some(cmd) = stream.next().await {
        match cmd {
            NavCommand::Push { route, ticket: None } if route.kind == "Home" => return,
            _ => continue,
        }
    }

    panic!("did not observe expected Home push command");
}

#[tokio::test]
async fn push_with_ticket_can_be_resolved() {
    oxide_core::init_navigation().unwrap();
    let runtime = oxide_core::navigation_runtime().unwrap();

    let route = NavRoute {
        kind: "Charts".into(),
        payload: serde_json::json!({}),
        extras: None,
    };

    let mut stream =
        tokio_stream::wrappers::BroadcastStream::new(runtime.subscribe_commands()).filter_map(|r| r.ok());

    let (ticket, rx) = runtime.push_with_ticket(route.clone()).await;

    while let Some(cmd) = stream.next().await {
        match cmd {
            NavCommand::Push { route: pushed, ticket: Some(t) } if t == ticket => {
                assert_eq!(pushed.kind, route.kind);
                break;
            }
            _ => continue,
        }
    }

    let resolved = runtime.emit_result(&ticket, serde_json::json!({"ok": true})).await;
    assert!(resolved);

    let value = rx.await.unwrap();
    assert_eq!(value, serde_json::json!({"ok": true}));
}
