use ast::ModuleUnit;
use compiler::Compiler;
use inkwell::context::Context;

mod ast;
mod compiler;
mod lexer;
mod parser;

fn main() {
    let file = std::fs::File::open("basic.js").unwrap();
    let _ = ModuleUnit::new("module_1".to_string(), file).unwrap();
    let context = Context::create();

    let _ = Compiler::new(&context);
}
