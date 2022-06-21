use ast::Module;

mod ast;
mod lexer;
// mod map;
mod parser;
mod precompiler;

fn main() {
    let in_file = std::fs::File::open("test_scripts/basic.js").unwrap();
    let _ = std::fs::File::create("test_scripts/basic.ll").unwrap();

    let module = Module::new("module_1".to_string(), in_file).unwrap();

    let _ = module.precompile();
}
