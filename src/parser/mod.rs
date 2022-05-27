use crate::lexer::{self, Token};
use thiserror::Error;

mod assigment_expression;
mod identifier;
mod literal;
mod right_assignment_value;
mod variable_declaration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token provided: {0}")]
    UnexpectedToken(Token),
    #[error("Parsing token error: {0}")]
    ParseTokenError(#[from] lexer::Error),
}
