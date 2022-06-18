use super::{Identifier, RightAssigmentValue};
use crate::{
    lexer::{CharReader, Token},
    parser::{self, Parser},
};
use std::io::Read;

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4"
#[derive(Debug, PartialEq)]
pub struct VariableDeclaration(pub AssigmentExpression);

/// AssigmentExpression - Expression type for variable assigment, like "a = 4"
#[derive(Debug, PartialEq)]
pub struct AssigmentExpression {
    pub left: Identifier,
    pub right: RightAssigmentValue,
}

impl Parser for AssigmentExpression {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        let left = Identifier::parse(cur_token, reader)?;

        match Token::get_token(reader)? {
            Token::Assign => {}
            token => return Err(parser::Error::UnexpectedToken(token)),
        }

        let right = RightAssigmentValue::parse(Token::get_token(reader)?, reader)?;
        Ok(Self { left, right })
    }
}

impl Parser for VariableDeclaration {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Var => Ok(Self(AssigmentExpression::parse(
                Token::get_token(reader)?,
                reader,
            )?)),
            token => Err(parser::Error::UnexpectedToken(token)),
        }
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

    #[test]
    fn variable_declaration_test() {
        let mut reader = CharReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(Token::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            VariableDeclaration(AssigmentExpression {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            })
        );

        let mut reader = CharReader::new("var name1 = name2;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(Token::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            VariableDeclaration(AssigmentExpression {
                left: Identifier {
                    name: "name1".to_string()
                },
                right: RightAssigmentValue::Identifier(Identifier {
                    name: "name2".to_string()
                })
            })
        );
    }
}
