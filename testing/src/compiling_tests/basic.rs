use crate::compile::CompileSuite;

#[test]
fn primitive_types_test() {
    CompileSuite::new(
        "../test_scripts/basic/primitive_types.js",
        "primitive_types",
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

#[test]
fn logical_not_test() {
    CompileSuite::new("../test_scripts/basic/logical_not.js", "logical_not")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}
