use js_ast::Module;

mod compiler;
mod js_ast;
mod llvm_ast;
mod parser;
mod precompiler;

fn main() {
    let in_file = std::fs::File::open("../test_scripts/basic.js").unwrap();
    let mut out_file = std::fs::File::create("../test_scripts/basic.ll").unwrap();
    let js_module = Module::new("module_1".to_string(), in_file).unwrap();
    let llvm_module = js_module.precompile().unwrap();

    llvm_module.compile_to(&mut out_file).unwrap()
}
