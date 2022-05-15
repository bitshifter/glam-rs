#[test]
fn check_macros_soundness() {
    let t = trybuild::TestCases::new();
    t.pass("tests/macros_soundness/should_compile.rs");
    t.compile_fail("tests/macros_soundness/should_fail.rs");
}
