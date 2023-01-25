use crate::CompileSuite;

#[test]
fn primitive_types_1_test() {
    let test = CompileSuite::new(
        "../test_scripts/basic/primitive_types_1.js",
        "primitive_types_1",
    )
    .compile()
    .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn primitive_types_2_test() {
    let test = CompileSuite::new(
        "../test_scripts/basic/primitive_types_2.js",
        "primitive_types_2",
    )
    .compile()
    .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn functions_test() {
    let test = CompileSuite::new("../test_scripts/basic/functions.js", "functions")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn block_test() {
    let test = CompileSuite::new("../test_scripts/basic/block.js", "block")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn if_else_test() {
    let test = CompileSuite::new("../test_scripts/basic/if_else.js", "if_else")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn loops_test() {
    let test = CompileSuite::new("../test_scripts/basic/loops.js", "loops")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn object_test() {
    let test = CompileSuite::new("../test_scripts/basic/object.js", "object")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn array_test() {
    let test = CompileSuite::new("../test_scripts/basic/array.js", "array")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn comments_test() {
    let test = CompileSuite::new("../test_scripts/basic/comments.js", "comments")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}
