use lexer::{CharReader, Token};

mod ast;
mod lexer;
mod parser;

fn main() {
    let file = std::fs::File::open("basic.js").unwrap();
    let mut reader = CharReader::new(file);

    let _ = Token::get_token(&mut reader);
}
