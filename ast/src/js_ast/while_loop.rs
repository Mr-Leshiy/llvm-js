use super::{BlockStatement, Identifier, VariableExpression};
use crate::{llvm_ast, Error};
use lexer::{Keyword, Separator, Token, TokenReader};
use precompiler::Precompiler;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub condition: VariableExpression,
    pub body: BlockStatement,
}

impl WhileLoop {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Keyword(Keyword::While) => match reader.next_token()? {
                Token::Separator(Separator::OpenBrace) => {
                    let condition = VariableExpression::parse(reader.next_token()?, reader)?;
                    match reader.next_token()? {
                        Token::Separator(Separator::CloseBrace) => {
                            let body = BlockStatement::parse(reader.next_token()?, reader)?;

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
}

impl WhileLoop {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::WhileLoop, precompiler::Error<Identifier>> {
        Ok(llvm_ast::WhileLoop {
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
        let mut reader = TokenReader::new("while (true) {}".as_bytes());
        assert_eq!(
            WhileLoop::parse(reader.next_token().unwrap(), &mut reader),
            Ok(WhileLoop {
                condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                body: BlockStatement { body: Vec::new() },
            })
        );
    }
}
