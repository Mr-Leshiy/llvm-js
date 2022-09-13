use super::{Expression, Identifier};
use crate::{llvm_ast, Error};
use lexer::{Token, TokenReader};
use precompiler::Precompiler;
use std::io::Read;

/// Program
#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Expression>,
}

impl Program {
    pub fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, Error> {
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

impl Program {
    pub fn precompile(
        self,
        mut precompiler: Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::Program, precompiler::Error<Identifier>> {
        let mut body = Vec::new();
        for expr in self.body {
            expr.precompile(&mut precompiler)?
                .into_iter()
                .for_each(|expr| body.push(expr));
        }

        Ok(llvm_ast::Program {
            functions: precompiler.function_declarations,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{VariableAssigment, VariableExpression, VariableValue};

    #[test]
    fn parse_program_test() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            Program::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Program {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: "name".to_string().into(),
                    right: Some(VariableExpression::VariableValue(VariableValue::Number(
                        12_f64
                    )))
                })]
            })
        );
    }
}
