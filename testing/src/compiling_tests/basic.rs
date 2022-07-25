use crate::compile::CompileSuite;

#[test]
fn variables_test() {
    let suite = CompileSuite::new("../test_scripts/basic/variables.js", "variables");

    suite.compile();
    suite.cleanup();
}
