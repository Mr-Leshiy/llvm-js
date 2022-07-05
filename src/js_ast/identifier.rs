use crate::{
    lexer::{Token, TokenReader},
    parser::{self, Parser},
};
use std::{fmt::Display, io::Read};

/// Identifier - Expression type for any identifier, like "name"
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier: {0}", self.name)
    }
}

impl Parser for Identifier {
    fn parse<R: Read>(cur_token: Token, _: &mut TokenReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Ident(name) => Ok(Self { name }),
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_indetifier_test() {
        let mut reader = TokenReader::new("name".as_bytes());
        assert_eq!(
            Identifier::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Identifier {
                name: "name".to_string()
            })
        );
    }
}
