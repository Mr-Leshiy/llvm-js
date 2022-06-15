use super::{Error, Parser};
use crate::{
    ast::{Expression, Program},
    lexer::{CharReader, Token},
};
use std::io::Read;

impl Parser for Program {
    fn parse<R: Read>(mut cur_token: Token, reader: &mut CharReader<R>) -> Result<Self, Error> {
        let mut body = Vec::new();

        loop {
            let expr = match cur_token {
                Token::Eof => break,
                cur_token => Expression::parse(cur_token, reader)?,
            };

            cur_token = Token::get_token(reader)?;
            body.push(expr);
        }

        Ok(Program { body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AssigmentExpression, Identifier, Literal, RightAssigmentValue};

    #[test]
    fn program_test() {
        let mut reader = CharReader::new("var name = 12;".as_bytes());
        assert_eq!(
            Program::parse(Token::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Program {
                body: vec![Expression::VariableDeclaration(AssigmentExpression {
                    left: Identifier {
                        name: "name".to_string()
                    },
                    right: RightAssigmentValue::Literal(Literal::Number(12_f64))
                })]
            }
        );
    }
}
