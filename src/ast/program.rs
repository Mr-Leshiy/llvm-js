use super::Expression;
use crate::{
    lexer::{self, CharReader, Token},
    parser::{self, Parser},
};
use std::io::Read;

/// Program
#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Expression>,
}

impl Parser for Program {
    fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut CharReader<R>,
    ) -> Result<Self, parser::Error> {
        let mut body = Vec::new();

        loop {
            let expr = match cur_token {
                Token::Eof => break,
                cur_token => Expression::parse(cur_token, reader)?,
            };

            cur_token = lexer::get_token(reader)?;
            body.push(expr);
        }

        Ok(Self { body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AssigmentExpression, Identifier, Literal, RightAssigmentValue};

    #[test]
    fn program_test() {
        let mut reader = CharReader::new("name = 12;".as_bytes());
        assert_eq!(
            Program::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Program {
                body: vec![Expression::Assigment(AssigmentExpression {
                    left: Identifier {
                        name: "name".to_string()
                    },
                    right: RightAssigmentValue::Literal(Literal::Number(12_f64))
                })]
            }
        );
    }
}
