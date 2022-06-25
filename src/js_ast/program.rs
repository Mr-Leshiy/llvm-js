use super::Expression;
use crate::{
    lexer::{self, CharReader, Token},
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
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

impl Precompile for Program {
    type Output = llvm_ast::Program;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        // first need to precompile program body
        let mut body = Vec::new();
        for expr in self.body {
            expr.precompile(precompiler)?
                .into_iter()
                .for_each(|expr| body.push(expr));
        }

        let mut functions = Vec::new();
        // TODO: need to optimize
        for func in precompiler.function_declarations.clone() {
            functions.push(func.precompile(precompiler)?);
        }

        Ok(llvm_ast::Program { functions, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{Identifier, Literal, RightAssigmentValue, VariableAssigment};

    #[test]
    fn parse_program_test() {
        let mut reader = CharReader::new("name = 12;".as_bytes());
        assert_eq!(
            Program::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            Program {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: Identifier {
                        name: "name".to_string()
                    },
                    right: RightAssigmentValue::Literal(Literal::Number(12_f64))
                })]
            }
        );
    }
}
