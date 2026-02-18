#![cfg(feature = "navigation-binding")]

use oxide_core::navigation::{
    NavCommand, NoExtra, NoReturn, OxideRoute, OxideRouteKind, OxideRoutePayload, Route,
};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum TestRouteKind {
    Home,
    Charts,
}

impl OxideRouteKind for TestRouteKind {
    fn as_str(&self) -> &'static str {
        match self {
            TestRouteKind::Home => "Home",
            TestRouteKind::Charts => "Charts",
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct TestRoute {
    kind: TestRouteKind,
}

impl Route for TestRoute {
    type Return = NoReturn;
    type Extra = NoExtra;
}

#[derive(Clone, Serialize, Deserialize)]
struct TestRoutePayload {
    kind: TestRouteKind,
}

impl OxideRoutePayload for TestRoutePayload {
    type Kind = TestRouteKind;

    fn kind(&self) -> Self::Kind {
        self.kind
    }
}

impl OxideRoute for TestRoute {
    type Payload = TestRoutePayload;

    fn into_payload(self) -> Self::Payload {
        TestRoutePayload { kind: self.kind }
    }
}

#[tokio::test]
async fn push_emits_a_command() {
    oxide_core::init_navigation().unwrap();
    let runtime = oxide_core::navigation_runtime().unwrap();

    let mut stream =
        tokio_stream::wrappers::BroadcastStream::new(runtime.subscribe_commands()).filter_map(|r| r.ok());

    runtime.push(TestRoute {
        kind: TestRouteKind::Home,
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

    let route = TestRoute {
        kind: TestRouteKind::Charts,
    };

    let mut stream =
        tokio_stream::wrappers::BroadcastStream::new(runtime.subscribe_commands()).filter_map(|r| r.ok());

    let (ticket, rx) = runtime.push_with_ticket(route.clone()).await;

    while let Some(cmd) = stream.next().await {
        match cmd {
            NavCommand::Push { route: pushed, ticket: Some(t) } if t == ticket => {
                assert_eq!(pushed.kind, "Charts");
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
