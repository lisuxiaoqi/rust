#[test]
fn test_entry() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic.rs");
    t.compile_fail("tests/fail.rs");
    t.compile_fail("tests/fail2.rs");
}
