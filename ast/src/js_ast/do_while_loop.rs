use super::{BlockStatement, VariableExpression};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Keyword, Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct DoWhileLoop {
    pub condition: VariableExpression,
    pub body: BlockStatement,
}

impl DoWhileLoop {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
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
                                token => Err(LexerError::UnexpectedToken(token)),
                            }
                        }
                        token => Err(LexerError::UnexpectedToken(token)),
                    },
                    token => Err(LexerError::UnexpectedToken(token)),
                }
            }
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }
}

impl DoWhileLoop {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::DoWhileLoop, PrecompilerError> {
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
}
