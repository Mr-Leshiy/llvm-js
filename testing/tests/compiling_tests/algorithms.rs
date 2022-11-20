use crate::CompileSuite;

#[test]
fn sorts_test() {
    CompileSuite::new("../test_scripts/algorithms/sorts.js", "sorts")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}
