use ast::ModuleUnit;
use compiler::Compiler;
use inkwell::context::Context;

mod ast;
mod compiler;
mod lexer;
mod map;
mod parser;
mod precompiler;

fn main() {
    let in_file = std::fs::File::open("test_scripts/basic.js").unwrap();
    let mut out_file = std::fs::File::create("test_scripts/basic.ll").unwrap();

    let module = ModuleUnit::new("module_1".to_string(), in_file).unwrap();

    let context = Context::create();
    let mut compiler = Compiler::new(&context);

    module.compile_to(&mut compiler, &mut out_file).unwrap();
}
