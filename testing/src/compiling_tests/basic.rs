use crate::compile::CompileSuite;

#[test]
fn primitive_types_1_test() {
    CompileSuite::new(
        "../test_scripts/basic/primitive_types_1.js",
        "primitive_types_1",
    )
    .compile()
    .unwrap()
    .run()
    .unwrap()
    .cleanup();
}

#[test]
fn primitive_types_2_test() {
    CompileSuite::new(
        "../test_scripts/basic/primitive_types_2.js",
        "primitive_types_2",
    )
    .compile()
    .unwrap()
    .run()
    .unwrap()
    .cleanup();
}

#[test]
fn functions_test() {
    CompileSuite::new("../test_scripts/basic/functions.js", "functions")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn block_test() {
    CompileSuite::new("../test_scripts/basic/block.js", "block")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn comments_test() {
    CompileSuite::new("../test_scripts/basic/comments.js", "comments")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}
