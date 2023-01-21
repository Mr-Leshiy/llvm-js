use crate::CompileSuite;

#[test]
fn primitive_types_test() {
    CompileSuite::new(
        "../test_scripts/memory/primitive_types.js",
        "primitive_types",
    )
    .compile()
    .unwrap()
    .run_with_valgrind()
    .unwrap()
    .cleanup();
}
