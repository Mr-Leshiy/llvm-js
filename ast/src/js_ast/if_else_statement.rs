use super::{BlockStatement, VariableExpression};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Keyword, Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct IfElseStatement {
    pub condition: VariableExpression,
    pub if_clause: BlockStatement,
    pub else_clause: BlockStatement,
}

impl IfElseStatement {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        match cur_token {
            Token::Keyword(Keyword::If) => match reader.next_token()? {
                Token::Separator(Separator::OpenBrace) => {
                    let condition = VariableExpression::parse(reader.next_token()?, reader)?;
                    match reader.next_token()? {
                        Token::Separator(Separator::CloseBrace) => {
                            let if_clause = BlockStatement::parse(reader.next_token()?, reader)?;

                            reader.start_saving();
                            let else_clause = match reader.next_token()? {
                                Token::Keyword(Keyword::Else) => {
                                    reader.reset_saving();
                                    BlockStatement::parse(reader.next_token()?, reader)?
                                }
                                _ => {
                                    reader.stop_saving();
                                    BlockStatement { body: Vec::new() }
                                }
                            };

                            Ok(Self {
                                condition,
                                if_clause,
                                else_clause,
                            })
                        }
                        token => Err(LexerError::UnexpectedToken(token)),
                    }
                }
                token => Err(LexerError::UnexpectedToken(token)),
            },
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }
}

impl IfElseStatement {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::IfElseStatement, PrecompilerError> {
        Ok(llvm_ast::IfElseStatement {
            condition: self.condition.precompile(precompiler)?,
            if_clause: self.if_clause.precompile(precompiler)?,
            else_clause: self.else_clause.precompile(precompiler)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::VariableValue;

    #[test]
    fn parse_if_else_statement_test() {
        let mut reader = TokenReader::new("if (true) {}".as_bytes());
        assert_eq!(
            IfElseStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(IfElseStatement {
                condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                if_clause: BlockStatement { body: Vec::new() },
                else_clause: BlockStatement { body: Vec::new() }
            })
        );

        let mut reader = TokenReader::new("if (true) {} else {}".as_bytes());
        assert_eq!(
            IfElseStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(IfElseStatement {
                condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
                if_clause: BlockStatement { body: Vec::new() },
                else_clause: BlockStatement { body: Vec::new() }
            })
        );
    }

    #[test]
    fn precompile_if_else_statement_test() {
        let mut precompiler = Precompiler::new(std::iter::empty());

        let if_else_statement = IfElseStatement {
            condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
            if_clause: BlockStatement { body: Vec::new() },
            else_clause: BlockStatement { body: Vec::new() },
        };
        assert_eq!(
            if_else_statement.precompile(&mut precompiler),
            Ok(llvm_ast::IfElseStatement {
                condition: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Boolean(true)
                ),
                if_clause: vec![],
                else_clause: vec![],
            })
        );

        let if_else_statement = IfElseStatement {
            condition: VariableExpression::VariableValue(VariableValue::Boolean(true)),
            if_clause: BlockStatement { body: Vec::new() },
            else_clause: BlockStatement { body: Vec::new() },
        };
        assert_eq!(
            if_else_statement.precompile(&mut precompiler),
            Ok(llvm_ast::IfElseStatement {
                condition: llvm_ast::VariableExpression::VariableValue(
                    llvm_ast::VariableValue::Boolean(true)
                ),
                if_clause: vec![],
                else_clause: vec![],
            })
        );
    }
}
