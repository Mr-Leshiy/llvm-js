use lexer::Token;
use precompiler::rpn;
use thiserror::Error;

pub mod js_ast;
pub mod llvm_ast;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Unexpected token provided: {0}")]
    UnexpectedToken(Token),
    #[error("Lexer error: {0}")]
    LexerError(#[from] lexer::Error),
    #[error("Rpn error: {0}")]
    RpnError(#[from] rpn::Error),
}
