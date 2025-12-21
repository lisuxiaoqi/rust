#[test]
pub fn entry() {
    let t = trybuild::TestCases::new();
    t.pass("tests/pass.rs");
}
