use super::{BlockStatement, Identifier, VariableExpression};
use crate::{llvm_ast, Error};
use lexer::{Keyword, Separator, Token, TokenReader};
use precompiler::Precompiler;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct DoWhileLoop {
    pub condition: VariableExpression,
    pub body: BlockStatement,
}

impl DoWhileLoop {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Keyword(Keyword::Do) => {
                let body = BlockStatement::parse(reader.next_token()?, reader)?;
                match reader.next_token()? {
                    Token::Keyword(Keyword::While) => match reader.next_token()? {
                        Token::Separator(Separator::OpenBrace) => {
                            let condition =
                                VariableExpression::parse(reader.next_token()?, reader)?;
                            match reader.next_token()? {
                                Token::Separator(Separator::CloseBrace) => {
                                    Ok(Self { condition, body })
                                }
                                token => Err(Error::UnexpectedToken(token)),
                            }
                        }
                        token => Err(Error::UnexpectedToken(token)),
                    },
                    token => Err(Error::UnexpectedToken(token)),
                }
            }
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

impl DoWhileLoop {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::DoWhileLoop, precompiler::Error<Identifier>> {
        Ok(llvm_ast::DoWhileLoop {
            condition: self.condition.precompile(precompiler)?,
            body: self.body.precompile(precompiler)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::VariableValue;

    #[test]
    fn parse_while_loop_test() {
        let mut reader = TokenReader::new("do {} while(true);".as_bytes());
        assert_eq!(
            DoWhileLoop::parse(reader.next_token().unwrap(), &mut reader),
            Ok(DoWhileLoop {
                condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                body: BlockStatement { body: Vec::new() },
            })
        );
    }

    #[test]
    fn precompile_while_loop_test() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let if_else_statement = DoWhileLoop {
            condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
            body: BlockStatement { body: Vec::new() },
        };
        assert_eq!(
            if_else_statement.precompile(&mut precompiler),
            Ok(llvm_ast::DoWhileLoop {
                condition: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Boolean(true)
                ),
                body: vec![],
            })
        );

        let if_else_statement = DoWhileLoop {
            condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
            body: BlockStatement { body: Vec::new() },
        };
        assert_eq!(
            if_else_statement.precompile(&mut precompiler),
            Ok(llvm_ast::DoWhileLoop {
                condition: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Boolean(true)
                ),
                body: vec![],
            })
        );
    }
}
