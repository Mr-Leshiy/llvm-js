use crate::compile::CompileSuite;

#[test]
fn logical_not_test() {
    CompileSuite::new("../test_scripts/logical/logical_not.js", "logical_not")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn logical_seq_test() {
    CompileSuite::new("../test_scripts/logical/logical_seq.js", "logical_seq")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}

#[test]
fn logical_sne_test() {
    CompileSuite::new("../test_scripts/logical/logical_sne.js", "logical_sne")
        .compile()
        .unwrap()
        .run()
        .unwrap()
        .cleanup();
}