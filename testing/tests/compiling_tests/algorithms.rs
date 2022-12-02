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

#[test]
fn double_linked_list_test() {
    CompileSuite::new(
        "../test_scripts/algorithms/double_linked_list.js",
        "double_linked_list",
    )
    .compile()
    .unwrap()
    .run()
    .unwrap()
    .cleanup();
}
