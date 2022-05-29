use ast::Program;
use lexer::{CharReader, Token};
use parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let file = std::fs::File::open("basic.js").unwrap();
    let mut reader = CharReader::new(file);

    let _ = Program::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap();
}
