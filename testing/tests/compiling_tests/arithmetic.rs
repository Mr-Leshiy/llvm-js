use crate::run_test;

#[test]
fn addition_test() {
    run_test("../test_scripts/arithmetic/addition.js", "addition");
}

#[test]
fn string_concat_test() {
    run_test(
        "../test_scripts/arithmetic/string_concat.js",
        "string_concat",
    );
}

#[test]
fn substraction_test() {
    run_test("../test_scripts/arithmetic/substraction.js", "substraction");
}

#[test]
fn multiplication_test() {
    run_test(
        "../test_scripts/arithmetic/multiplication.js",
        "multiplication",
    );
}

#[test]
fn division_test() {
    run_test("../test_scripts/arithmetic/division.js", "division");
}
