use ast::js_ast::Module;
use compiler::predefined_functions::{
    test::{AssertEqFn, AssertFn, GbVariablesCount, PrintFn},
    PredefineFunctionName,
};

fn main() {
    let in_file = std::fs::File::open("test_scripts/basic.js").unwrap();
    let mut out_file = std::fs::File::create("test_scripts/basic.ll").unwrap();
    let extern_functions = vec![
        PrintFn::NAME.to_string(),
        AssertFn::NAME.to_string(),
        AssertEqFn::NAME.to_string(),
        GbVariablesCount::NAME.to_string(),
    ];

    Module::new("module_1".to_string(), in_file)
        .unwrap()
        .precompile(extern_functions.into_iter().map(|e| e.into()))
        .unwrap()
        .compile_to(&mut out_file)
        .unwrap();
}
