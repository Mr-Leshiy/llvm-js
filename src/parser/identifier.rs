use super::{Error, Parser};
use crate::{
    ast::Identifier,
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for Identifier {
    fn parse<R: Read>(cur_token: Token, _: &mut CharReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Ident(name) => Ok(Identifier { name }),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indetifier_test() {
        let mut reader = CharReader::new("name".as_bytes());
        assert_eq!(
            Identifier::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Identifier {
                name: "name".to_string()
            }
        );
    }
}
