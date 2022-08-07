use super::{
    BlockStatement, FunctionCall, FunctionDeclaration, Identifier, VariableAssigment,
    VariableDeclaration,
};
use crate::llvm_ast;
use lexer::{Keyword, Parser, Separator, Token, TokenReader};
use precompiler::{self, Precompiler};
use std::{fmt::Debug, io::Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
    BlockStatement(BlockStatement),
}

impl Parser for Expression {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        match cur_token {
            Token::Keyword(Keyword::Function) => Ok(Self::FunctionDeclaration(
                FunctionDeclaration::parse(cur_token, reader)?,
            )),
            Token::Keyword(Keyword::Var) => Ok(Self::VariableDeclaration(
                VariableDeclaration::parse(cur_token, reader)?,
            )),
            Token::Ident(_) => {
                reader.start_saving();
                match FunctionCall::parse(cur_token.clone(), reader) {
                    Ok(res) => {
                        reader.reset_saving();
                        Ok(Self::FunctionCall(res))
                    }
                    Err(_) => {
                        reader.stop_saving();
                        Ok(Self::VariableAssigment(VariableAssigment::parse(
                            cur_token, reader,
                        )?))
                    }
                }
            }
            Token::Separator(Separator::OpenCurlyBrace) => Ok(Self::BlockStatement(
                BlockStatement::parse(cur_token, reader)?,
            )),
            token => Err(lexer::Error::UnexpectedToken(token)),
        }
    }
}

impl Expression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<Vec<llvm_ast::Expression>, precompiler::Error<Identifier>> {
        match self {
            Expression::FunctionDeclaration(function_declaration) => {
                let function_declaration = function_declaration.precompile(precompiler)?;
                precompiler.function_declarations.push(function_declaration);
                Ok(Vec::new())
            }
            Expression::FunctionCall(function_call) => {
                Ok(vec![llvm_ast::Expression::FunctionCall(
                    function_call.precompile(precompiler)?,
                )])
            }
            Expression::VariableDeclaration(variable_declaration) => {
                Ok(vec![llvm_ast::Expression::VariableDeclaration(
                    variable_declaration.precompile(precompiler)?,
                )])
            }
            Expression::VariableAssigment(variable_assigment) => {
                Ok(vec![llvm_ast::Expression::VariableAssigment(
                    variable_assigment.precompile(precompiler)?,
                )])
            }
            Expression::BlockStatement(block_statement) => block_statement.precompile(precompiler),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{VariableExpression, VariableValue};

    #[test]
    fn parse_expression_test1() {
        let mut reader = TokenReader::new("var name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableDeclaration(VariableDeclaration(
                VariableAssigment {
                    left: "name".to_string().into(),
                    right: VariableExpression::VariableValue(VariableValue::Number(12_f64))
                }
            )))
        );
    }

    #[test]
    fn parse_expression_test2() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableAssigment(VariableAssigment {
                left: "name".to_string().into(),
                right: VariableExpression::VariableValue(VariableValue::Number(12_f64))
            })
        );
    }

    #[test]
    fn parse_expression_test3() {
        let mut reader = TokenReader::new("{ }".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement(BlockStatement { body: vec![] })
        );

        let mut reader = TokenReader::new("{ name1 = name2; }".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement(BlockStatement {
                body: vec![Expression::VariableAssigment(VariableAssigment {
                    left: "name1".to_string().into(),
                    right: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name2".to_string().into()
                    ))
                })]
            })
        );

        let mut reader =
            TokenReader::new("{ name1 = name2; { name1 = name2; name1 = name2; } }".as_bytes());

        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::BlockStatement(BlockStatement {
                body: vec![
                    Expression::VariableAssigment(VariableAssigment {
                        left: "name1".to_string().into(),
                        right: VariableExpression::VariableValue(VariableValue::Identifier(
                            "name2".to_string().into()
                        ))
                    }),
                    Expression::BlockStatement(BlockStatement {
                        body: vec![
                            Expression::VariableAssigment(VariableAssigment {
                                left: "name1".to_string().into(),
                                right: VariableExpression::VariableValue(
                                    VariableValue::Identifier("name2".to_string().into())
                                )
                            }),
                            Expression::VariableAssigment(VariableAssigment {
                                left: "name1".to_string().into(),
                                right: VariableExpression::VariableValue(
                                    VariableValue::Identifier("name2".to_string().into())
                                )
                            }),
                        ]
                    })
                ]
            })
        );
    }

    #[test]
    fn parse_expression_test4() {
        let mut reader = TokenReader::new("foo(a, b); a = 6;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::FunctionCall(FunctionCall {
                name: "foo".to_string().into(),
                args: vec![
                    VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into()
                    ))
                ]
            })
        );
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableAssigment(VariableAssigment {
                left: "a".to_string().into(),
                right: VariableExpression::VariableValue(VariableValue::Number(6_f64))
            }))
        );
    }
}
