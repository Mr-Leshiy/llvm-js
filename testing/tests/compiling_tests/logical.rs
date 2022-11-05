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
fn seq_test() {
    CompileSuite::new("../test_scripts/logical/seq.js", "seq")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn sne_test() {
    CompileSuite::new("../test_scripts/logical/sne.js", "sne")
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
    CompileSuite::new("../test_scripts/logical/lt.js", "lt")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn le_test() {
    CompileSuite::new("../test_scripts/logical/le.js", "le")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}
