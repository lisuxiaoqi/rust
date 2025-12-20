#[test]
fn test_entry() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass.rs");
}
