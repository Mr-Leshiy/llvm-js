use crate::lexer::{self, CharReader, Token};
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token provided: {0}")]
    UnexpectedToken(Token),
    #[error("Parsing token error: {0}")]
    ParseTokenError(#[from] lexer::Error),
}

pub trait Parser: Sized {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error>;
}
