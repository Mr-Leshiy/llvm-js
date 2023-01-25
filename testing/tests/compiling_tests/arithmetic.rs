use crate::CompileSuite;

#[test]
fn addition_test() {
    let test = CompileSuite::new("../test_scripts/arithmetic/addition.js", "addition")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn string_concat_test() {
    let test = CompileSuite::new(
        "../test_scripts/arithmetic/string_concat.js",
        "string_concat",
    )
    .compile()
    .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn substraction_test() {
    let test = CompileSuite::new("../test_scripts/arithmetic/substraction.js", "substraction")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn multiplication_test() {
    let test = CompileSuite::new(
        "../test_scripts/arithmetic/multiplication.js",
        "multiplication",
    )
    .compile()
    .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn division_test() {
    let test = CompileSuite::new("../test_scripts/arithmetic/division.js", "division")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}
