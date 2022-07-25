use crate::compile::CompileSuite;

#[test]
fn basis_test() {
    let suite = CompileSuite::new("../test_scripts/basic.js", "basic");

    suite.compile();
    suite.cleanup();
}
