use crate::run_test;

#[test]
fn not_test() {
    run_test("../test_scripts/logical/not.js", "not");
}

#[test]
fn and_test() {
    run_test("../test_scripts/logical/and.js", "and");
}

#[test]
fn logical_or_test() {
    run_test("../test_scripts/logical/or.js", "or");
}

#[test]
fn gt_test() {
    run_test("../test_scripts/logical/gt.js", "gt");
}

#[test]
fn ge_test() {
    run_test("../test_scripts/logical/ge.js", "ge");
}

#[test]
fn lt_test() {
    run_test("../test_scripts/logical/lt.js", "lt");
}

#[test]
fn le_test() {
    run_test("../test_scripts/logical/le.js", "le");
}
