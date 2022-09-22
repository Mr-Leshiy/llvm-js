use crate::compile::CompileSuite;

#[test]
fn addition_test() {
    CompileSuite::new("../test_scripts/arithmetic/addition.js", "addition")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn substraction_test() {
    CompileSuite::new("../test_scripts/arithmetic/substraction.js", "substraction")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn multiplication_test() {
    CompileSuite::new(
        "../test_scripts/arithmetic/multiplication.js",
        "multiplication",
    )
    .compile()
    .unwrap()
    .run()
    .unwrap()
    .cleanup();
}

#[test]
fn division_test() {
    CompileSuite::new("../test_scripts/arithmetic/division.js", "division")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}
