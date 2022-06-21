use super::AssigmentExpression;
use crate::{
    lexer::{self, CharReader, Keyword, Token},
    parser::{self, Parser},
};
use std::io::Read;

/// VariableDeclaration - Expression type for variable assigment, like "var a = 4"
#[derive(Debug, PartialEq)]
pub struct VariableDeclaration(pub AssigmentExpression);

impl Parser for VariableDeclaration {
    fn parse<R: Read>(cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Keyword(Keyword::Var) => Ok(Self(AssigmentExpression::parse(
                lexer::get_token(reader)?,
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
    fn parse_variable_declaration_test() {
        let mut reader = CharReader::new("var name = 12;".as_bytes());
        assert_eq!(
            VariableDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
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
            VariableDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
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
