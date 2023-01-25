use crate::CompileSuite;

#[test]
fn not_test() {
    CompileSuite::new("../test_scripts/logical/not.js", "not")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn and_test() {
    CompileSuite::new("../test_scripts/logical/and.js", "and")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn logical_or_test() {
    CompileSuite::new("../test_scripts/logical/or.js", "or")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn gt_test() {
    CompileSuite::new("../test_scripts/logical/gt.js", "gt")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn ge_test() {
    CompileSuite::new("../test_scripts/logical/ge.js", "ge")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn lt_test() {
    let test = CompileSuite::new("../test_scripts/logical/lt.js", "lt")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}

#[test]
fn le_test() {
    let test = CompileSuite::new("../test_scripts/logical/le.js", "le")
        .compile()
        .unwrap();
    #[cfg(not(feature = "mem-check"))]
    test.run().unwrap().cleanup();
    #[cfg(all(target_os = "linux", feature = "mem-check"))]
    test.run_with_valgrind().unwrap().cleanup();
}
