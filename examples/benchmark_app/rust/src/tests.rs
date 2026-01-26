use crate::api::{counter_bridge, json_bridge, sieve_bridge};
use crate::state::counter_action::CounterAction;
use crate::state::json_action::JsonAction;
use crate::state::sieve_action::SieveAction;

const EXPECTED_COUNTER_CHECKSUM_AFTER_1000: u64 = 0x01f4b8025ff13a2c;
const EXPECTED_SIEVE_CHECKSUM_AFTER_1: u64 = 0x61712022e7704885;
const EXPECTED_JSON_LIGHT_CHECKSUM_AFTER_1: u64 = 0x1b107b9bc4545255;
const EXPECTED_JSON_HEAVY_CHECKSUM_AFTER_1: u64 = 0xb067dca7d88da466;

#[tokio::test]
async fn counter_dispatch_updates_state_and_revision() {
    let engine = counter_bridge::create_engine();
    let before = counter_bridge::current(&engine).await;
    assert_eq!(before.state.counter, 0);
    assert_eq!(before.revision, 0);

    let after = counter_bridge::dispatch(&engine, CounterAction::Run { iterations: 1000 })
        .await
        .expect("dispatch");
    assert_eq!(after.state.counter, 1000);
    assert_eq!(after.revision, 1);
    assert_eq!(after.state.checksum, EXPECTED_COUNTER_CHECKSUM_AFTER_1000);
}

#[tokio::test]
async fn json_dispatch_updates_state_and_revision() {
    let engine = json_bridge::create_engine();
    let before = json_bridge::current(&engine).await;
    assert_eq!(before.state.counter, 0);
    assert_eq!(before.revision, 0);

    let after = json_bridge::dispatch(&engine, JsonAction::RunLight { iterations: 1 })
        .await
        .expect("dispatch");
    assert_eq!(after.state.counter, 1);
    assert_eq!(after.revision, 1);
    assert_eq!(after.state.checksum, EXPECTED_JSON_LIGHT_CHECKSUM_AFTER_1);

    let heavy_engine = json_bridge::create_engine();
    let after_heavy = json_bridge::dispatch(&heavy_engine, JsonAction::RunHeavy { iterations: 1 })
        .await
        .expect("dispatch heavy");
    assert_eq!(after_heavy.state.counter, 1);
    assert_eq!(after_heavy.revision, 1);
    assert_eq!(after_heavy.state.checksum, EXPECTED_JSON_HEAVY_CHECKSUM_AFTER_1);
}

#[tokio::test]
async fn sieve_dispatch_updates_state_and_revision() {
    let engine = sieve_bridge::create_engine();
    let before = sieve_bridge::current(&engine).await;
    assert_eq!(before.state.counter, 0);
    assert_eq!(before.revision, 0);

    let after = sieve_bridge::dispatch(&engine, SieveAction::Run { iterations: 1 })
        .await
        .expect("dispatch");
    assert_eq!(after.state.counter, 1);
    assert_eq!(after.revision, 1);
    assert_eq!(after.state.checksum, EXPECTED_SIEVE_CHECKSUM_AFTER_1);
}
