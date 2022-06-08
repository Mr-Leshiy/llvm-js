use ast::ModuleUnit;
use compiler::Compiler;
use inkwell::context::Context;

mod ast;
mod compiler;
mod lexer;
mod parser;

fn main() {
    let file = std::fs::File::open("test_scripts/basic.js").unwrap();

    let module = ModuleUnit::new("module_1".to_string(), file).unwrap();

    let context = Context::create();
    let mut compiler = Compiler::new(&context);

    module.compile_to(&mut compiler).unwrap();
}
