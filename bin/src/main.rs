use compiler::extern_functions::{printf::PrintFn, ExternFunctionName};
use ast::js_ast::Module;

fn main() {
    let in_file = std::fs::File::open("test_scripts/basic.js").unwrap();
    let mut out_file = std::fs::File::create("test_scripts/basic.ll").unwrap();
    let js_module = Module::new("module_1".to_string(), in_file).unwrap();
    let extern_functions = vec![PrintFn::NAME.to_string()];

    let llvm_module = js_module
        .precompile(extern_functions.clone().into_iter().map(|e| e.into()))
        .unwrap();

    llvm_module
        .compile_to(&mut out_file, extern_functions.into_iter())
        .unwrap()
}
