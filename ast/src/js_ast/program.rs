use super::Expression;
use lexer::{Parser, Token, TokenReader};
use std::io::Read;

/// Program
#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Expression>,
}

impl Parser for Program {
    fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, lexer::Error> {
        let mut body = Vec::new();

        loop {
            let expr = match cur_token {
                Token::Eof => break,
                cur_token => Expression::parse(cur_token, reader)?,
            };

            cur_token = reader.next_token()?;
            body.push(expr);
        }

        Ok(Self { body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{Literal, RightAssigmentValue, VariableAssigment};

    #[test]
    fn parse_program_test() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            Program::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Program {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: "name".to_string().into(),
                    right: RightAssigmentValue::Literal(Literal::Number(12_f64))
                })]
            })
        );
    }
}
