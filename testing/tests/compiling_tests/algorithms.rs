use crate::CompileSuite;

#[test]
fn sorts_test() {
    let test = CompileSuite::new("../test_scripts/algorithms/sorts.js", "sorts")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn double_linked_list_test() {
    let test = CompileSuite::new(
        "../test_scripts/algorithms/double_linked_list.js",
        "double_linked_list",
    )
    .compile()
    .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}
