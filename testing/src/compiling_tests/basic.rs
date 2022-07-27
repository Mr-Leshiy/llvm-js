use crate::compile::CompileSuite;

#[test]
fn variables_test() {
    CompileSuite::new("../test_scripts/basic/variables.js", "variables")
        .compile()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn functions_test() {
    CompileSuite::new("../test_scripts/basic/functions.js", "functions")
        .compile()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn block_test() {
    CompileSuite::new("../test_scripts/basic/block.js", "block")
        .compile()
        .run()
        .unwrap()
        .cleanup();
}
