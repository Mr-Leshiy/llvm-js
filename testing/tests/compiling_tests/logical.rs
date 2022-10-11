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
