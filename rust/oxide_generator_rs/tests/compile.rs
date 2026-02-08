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
    t.compile_fail("tests/ui/fail_reducer_wrong_signature.rs");
    t.compile_fail("tests/ui/fail_reducer_missing_args.rs");
    t.compile_fail("tests/ui/fail_reducer_missing_sideeffect.rs");
    t.compile_fail("tests/ui/fail_reducer_async_reduce.rs");
    t.compile_fail("tests/ui/fail_reducer_infer_without_sliced_state.rs");
    t.compile_fail("tests/ui/fail_reducer_infer_glob_import_without_sliced_state.rs");
}
