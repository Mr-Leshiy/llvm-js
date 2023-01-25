use crate::run_test;

#[test]
fn primitive_types_1_test() {
    run_test(
        "../test_scripts/basic/primitive_types_1.js",
        "primitive_types_1",
    );
}

#[test]
fn primitive_types_2_test() {
    run_test(
        "../test_scripts/basic/primitive_types_2.js",
        "primitive_types_2",
    );
}

#[test]
fn functions_test() {
    run_test("../test_scripts/basic/functions.js", "functions");
}

#[test]
fn block_test() {
    run_test("../test_scripts/basic/block.js", "block");
}

#[test]
fn if_else_test() {
    run_test("../test_scripts/basic/if_else.js", "if_else");
}

#[test]
fn loops_test() {
    run_test("../test_scripts/basic/loops.js", "loops");
}

#[test]
fn object_test() {
    run_test("../test_scripts/basic/object.js", "object");
}

#[test]
fn array_test() {
    run_test("../test_scripts/basic/array.js", "array");
}

#[test]
fn comments_test() {
    run_test("../test_scripts/basic/comments.js", "comments");
}
