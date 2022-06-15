use super::{Error, Parser};
use crate::{
    ast::{AssigmentExpression, Identifier, RightAssigmentValue},
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for AssigmentExpression {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        let left = Identifier::parse(cur_token, reader)?;

        match Token::get_token(reader)? {
            Token::Assign => {}
            token => return Err(Error::UnexpectedToken(token)),
        }

        let right = RightAssigmentValue::parse(Token::get_token(reader)?, reader)?;
        Ok(AssigmentExpression { left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Identifier, Literal, RightAssigmentValue};

    #[test]
    fn assigment_expression_test() {
        let mut reader = CharReader::new("name = 12;".as_bytes());
        assert_eq!(
            AssigmentExpression::parse(Token::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            AssigmentExpression {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            }
        );

        let mut reader = CharReader::new("name1 = name2;".as_bytes());
        assert_eq!(
            AssigmentExpression::parse(Token::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            AssigmentExpression {
                left: Identifier {
                    name: "name1".to_string()
                },
                right: RightAssigmentValue::Identifier(Identifier {
                    name: "name2".to_string()
                })
            }
        );
    }
}
