use crate::compile::CompileSuite;

#[test]
fn variables_test() {
    CompileSuite::new("../test_scripts/basic/variables.js", "variables")
        .compile()
        .cleanup();
}

#[test]
fn functions_test() {
    CompileSuite::new("../test_scripts/basic/functions.js", "functions")
        .compile()
        .cleanup();
}
