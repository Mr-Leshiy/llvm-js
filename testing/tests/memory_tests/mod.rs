use crate::CompileSuite;

#[test]
fn basic_test() {
    CompileSuite::new("../test_scripts/memory/basic.js", "basic")
        .compile()
        .unwrap()
        .run_with_valgrind()
        .unwrap()
        .cleanup();
}
