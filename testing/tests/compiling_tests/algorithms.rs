use crate::run_test;

#[test]
fn sorts_test() {
    run_test("../test_scripts/algorithms/sorts.js", "sorts");
}

#[test]
fn double_linked_list_test() {
    run_test(
        "../test_scripts/algorithms/double_linked_list.js",
        "double_linked_list",
    );
}
