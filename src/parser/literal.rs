use super::{Error, Parser};
use crate::{
    ast::Literal,
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for Literal {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Number(val) => Ok(Literal::Number(val)),
            Token::String(val) => Ok(Literal::String(val)),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigment_expression_test() {
        let mut reader = CharReader::new("12;".as_bytes());
        assert_eq!(
            Literal::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Literal::Number(12_f64)
        );

        let mut reader = CharReader::new(r#""name""#.as_bytes());
        assert_eq!(
            Literal::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Literal::String("name".to_string())
        );
    }
}
