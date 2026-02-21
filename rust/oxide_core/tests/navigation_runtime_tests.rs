#![cfg(feature = "navigation-binding")]

use oxide_core::navigation::{
    DefaultExtra, DefaultReturn, NavCommand, NavRoute, NoExtra, NoReturn, OxideRoute,
    OxideRouteKind, OxideRoutePayload, Route, RouteContext,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
struct TestExtra {
    label: String,
}

impl DefaultExtra for TestExtra {
    fn default_extra() -> Self {
        Self {
            label: "default".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct RouteWithExtras {
    kind: TestRouteKind,
    extras: TestExtra,
}

impl Route for RouteWithExtras {
    type Return = NoReturn;
    type Extra = TestExtra;

    fn extras(&self) -> Option<Self::Extra> {
        Some(self.extras.clone())
    }
}

impl OxideRoute for RouteWithExtras {
    type Payload = TestRoutePayload;

    fn into_payload(self) -> Self::Payload {
        TestRoutePayload { kind: self.kind }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct WrappedPayload {
    kind: TestRouteKind,
    payload: serde_json::Value,
}

impl OxideRoutePayload for WrappedPayload {
    type Kind = TestRouteKind;

    fn kind(&self) -> Self::Kind {
        self.kind
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct RouteDefaults;

impl Route for RouteDefaults {
    type Return = NoReturn;
    type Extra = NoExtra;
}

#[test]
fn default_return_values_are_provided() {
    assert_eq!(<bool as DefaultReturn>::default_return(), false);
    assert_eq!(<i32 as DefaultReturn>::default_return(), 0);
    assert_eq!(<i64 as DefaultReturn>::default_return(), 0);
    assert_eq!(<u32 as DefaultReturn>::default_return(), 0);
    assert_eq!(<u64 as DefaultReturn>::default_return(), 0);
    assert_eq!(<String as DefaultReturn>::default_return(), String::new());
    let value: Option<u32> = DefaultReturn::default_return();
    assert_eq!(value, None);
}

#[test]
fn default_extra_values_are_provided() {
    let extra = <TestExtra as DefaultExtra>::default_extra();
    assert_eq!(
        extra,
        TestExtra {
            label: "default".to_string()
        }
    );
    let _ = <NoExtra as DefaultExtra>::default_extra();
}

#[test]
fn route_defaults_return_empty_values() {
    assert_eq!(<RouteDefaults as Route>::path(), None);
    let route = RouteDefaults;
    let params: HashMap<&'static str, String> = route.params();
    let query: HashMap<&'static str, String> = route.query();
    assert!(params.is_empty());
    assert!(query.is_empty());
    assert!(route.extras().is_none());
}

#[tokio::test]
async fn push_emits_a_command() {
    let runtime = oxide_core::NavigationRuntime::new();

    let mut rx = runtime.subscribe_commands().unwrap();

    runtime
        .push(TestRoute {
        kind: TestRouteKind::Home,
    })
        .unwrap();

    let cmd = tokio::time::timeout(Duration::from_secs(1), rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(cmd, NavCommand::Push { route, ticket: None } if route.kind == "Home"));
}

#[tokio::test]
async fn push_with_ticket_can_be_resolved() {
    let runtime = oxide_core::NavigationRuntime::new();

    let route = TestRoute {
        kind: TestRouteKind::Charts,
    };

    let mut rx = runtime.subscribe_commands().unwrap();

    let (ticket, result_rx) = runtime.push_with_ticket(route.clone()).await.unwrap();

    let cmd = tokio::time::timeout(Duration::from_secs(1), rx.recv())
        .await
        .unwrap()
        .unwrap();
    match cmd {
        NavCommand::Push {
            route: pushed,
            ticket: Some(t),
        } if t == ticket => {
            assert_eq!(pushed.kind, "Charts");
        }
        other => panic!("unexpected command: {other:?}"),
    }

    let resolved = runtime.emit_result(&ticket, serde_json::json!({"ok": true})).await;
    assert!(resolved);

    let value = result_rx.await.unwrap();
    assert_eq!(value, serde_json::json!({"ok": true}));
}

#[tokio::test]
async fn pop_and_pop_until_emit_commands() {
    let runtime = oxide_core::NavigationRuntime::new();

    let mut rx = runtime.subscribe_commands().unwrap();

    runtime.pop().unwrap();
    runtime.pop_until(TestRouteKind::Home).unwrap();

    let first = tokio::time::timeout(Duration::from_secs(1), rx.recv())
        .await
        .unwrap()
        .unwrap();
    let second = tokio::time::timeout(Duration::from_secs(1), rx.recv())
        .await
        .unwrap()
        .unwrap();

    assert!(matches!(first, NavCommand::Pop { result: None }));
    assert!(matches!(second, NavCommand::PopUntil { kind } if kind == "Home"));
}

#[tokio::test]
async fn pop_with_json_emits_result() {
    let runtime = oxide_core::NavigationRuntime::new();

    let mut rx = runtime.subscribe_commands().unwrap();

    runtime
        .pop_with_json(serde_json::json!({"ok": true}))
        .unwrap();

    let cmd = tokio::time::timeout(Duration::from_secs(1), rx.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(cmd, NavCommand::Pop { result: Some(_) }));
}

#[tokio::test]
async fn reset_emits_payload_without_envelope() {
    let runtime = oxide_core::NavigationRuntime::new();
    let mut rx = runtime.subscribe_commands().unwrap();

    runtime
        .reset(vec![WrappedPayload {
        kind: TestRouteKind::Charts,
        payload: serde_json::json!({"id": 7}),
    }])
        .unwrap();

    let cmd = tokio::time::timeout(Duration::from_secs(1), rx.recv())
        .await
        .unwrap()
        .unwrap();

    match cmd {
        NavCommand::Reset { routes } => {
            assert_eq!(routes.len(), 1);
            assert_eq!(routes[0].kind, "Charts");
            assert_eq!(routes[0].payload, serde_json::json!({"id": 7}));
        }
        _ => panic!("expected reset command"),
    }
}

#[tokio::test]
async fn route_extras_are_encoded_in_push() {
    let runtime = oxide_core::NavigationRuntime::new();
    let mut rx = runtime.subscribe_commands().unwrap();

    runtime
        .push(RouteWithExtras {
        kind: TestRouteKind::Home,
        extras: TestExtra {
            label: "hello".to_string(),
        },
    })
        .unwrap();

    let cmd = tokio::time::timeout(Duration::from_secs(1), rx.recv())
        .await
        .unwrap()
        .unwrap();

    match cmd {
        NavCommand::Push { route, ticket: None } => {
            assert_eq!(route.kind, "Home");
            assert_eq!(route.extras, Some(serde_json::json!({"label": "hello"})));
        }
        _ => panic!("expected push command"),
    }
}

#[test]
fn set_current_route_updates_context() {
    let runtime = oxide_core::NavigationRuntime::new();
    let _rx = runtime.subscribe_route_context();
    runtime.set_current_route(Some(NavRoute {
        kind: "Home".to_string(),
        payload: serde_json::json!({"id": 1}),
        extras: None,
    }));

    let context = runtime.current_route_context();
    assert_eq!(
        context.current,
        Some(NavRoute {
            kind: "Home".to_string(),
            payload: serde_json::json!({"id": 1}),
            extras: None,
        })
    );
}

#[tokio::test]
async fn navigation_ctx_emits_commands_and_exposes_route() {
    let runtime = oxide_core::NavigationRuntime::new();
    let context = RouteContext {
        current: Some(NavRoute {
            kind: "Charts".to_string(),
            payload: serde_json::json!({"id": 3}),
            extras: None,
        }),
    };
    let nav = oxide_core::NavigationCtx::new(&runtime, &context);

    assert_eq!(nav.route(), &context);

    let mut rx = runtime.subscribe_commands().unwrap();

    nav.push(TestRoute {
        kind: TestRouteKind::Home,
    })
    .unwrap();
    nav.pop().unwrap();
    nav.pop_until(TestRouteKind::Charts).unwrap();

    let mut seen = Vec::new();
    while seen.len() < 3 {
        let cmd = tokio::time::timeout(Duration::from_secs(1), rx.recv())
            .await
            .unwrap()
            .unwrap();
        seen.push(cmd);
    }

    assert!(seen.iter().any(|c| matches!(c, NavCommand::Push { route, ticket: None } if route.kind == "Home")));
    assert!(seen.iter().any(|c| matches!(c, NavCommand::Pop { result: None })));
    assert!(seen.iter().any(|c| matches!(c, NavCommand::PopUntil { kind } if kind == "Charts")));
}
