#[test]
fn ui() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/pass_state_struct.rs");
    t.pass("tests/ui/pass_state_enum.rs");
    t.pass("tests/ui/pass_state_sliced_struct.rs");
    t.pass("tests/ui/pass_actions_enum.rs");
    t.pass("tests/ui/pass_reducer_fn.rs");
    t.pass("tests/ui/pass_reducer_infer_glob_import.rs");
    if cfg!(feature = "state-persistence") {
        t.pass("tests/ui/pass_reducer_persistence.rs");
    }

    t.compile_fail("tests/ui/fail_state_wrong_target.rs");
    t.compile_fail("tests/ui/fail_state_sliced_enum.rs");
    t.compile_fail("tests/ui/fail_actions_wrong_target.rs");
    t.compile_fail("tests/ui/fail_routes_wrong_target.rs");
    t.compile_fail("tests/ui/fail_routes_unexpected_args.rs");
    t.compile_fail("tests/ui/fail_oxide_route_wrong_target.rs");
    t.compile_fail("tests/ui/fail_oxide_route_unknown_arg.rs");
    t.compile_fail("tests/ui/fail_oxide_route_missing_equals.rs");
    t.compile_fail("tests/ui/fail_oxide_route_missing_value.rs");
    t.compile_fail("tests/ui/fail_oxide_route_missing_comma.rs");
    t.compile_fail("tests/ui/fail_reducer_wrong_signature.rs");
    t.compile_fail("tests/ui/fail_reducer_missing_args.rs");
    t.compile_fail("tests/ui/fail_reducer_missing_sideeffect.rs");
    t.compile_fail("tests/ui/fail_reducer_async_reduce.rs");
    t.compile_fail("tests/ui/fail_reducer_infer_without_sliced_state.rs");
    t.compile_fail("tests/ui/fail_reducer_infer_glob_import_without_sliced_state.rs");
    if cfg!(feature = "isolated-channels") {
        t.compile_fail("tests/ui/fail_oxide_event_channel_wrong_target.rs");
        t.compile_fail("tests/ui/fail_oxide_callback_wrong_target.rs");
    }

}
