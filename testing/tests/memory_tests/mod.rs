use crate::CompileSuite;

#[test]
fn primitive_types_test() {
    CompileSuite::new(
        "../test_scripts/memory/basic.js",
        "basic",
    )
    .compile()
    .unwrap()
    .run_with_valgrind()
    .unwrap()
    .cleanup();
}
