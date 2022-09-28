use super::{
    return_statement::ReturnStatement, BlockStatement, FunctionCall, FunctionDeclaration,
    Identifier, IfElseStatement, VariableAssigment, VariableDeclaration, WhileLoop, DoWhileLoop,
};
use crate::{llvm_ast, Error};
use lexer::{Keyword, Separator, Token, TokenReader};
use precompiler::Precompiler;
use std::{fmt::Debug, io::Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
    BlockStatement(BlockStatement),
    IfElseStatement(IfElseStatement),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
    ReturnStatement(ReturnStatement),
}

impl Expression {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Keyword(Keyword::Function) => Ok(Self::FunctionDeclaration(
                FunctionDeclaration::parse(cur_token, reader)?,
            )),
            Token::Keyword(Keyword::Var) => Ok(Self::VariableDeclaration(
                VariableDeclaration::parse(cur_token, reader)?,
            )),
            Token::Keyword(Keyword::Let) => Ok(Self::VariableDeclaration(
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
            Token::Keyword(Keyword::If) => Ok(Self::IfElseStatement(IfElseStatement::parse(
                cur_token, reader,
            )?)),
            Token::Keyword(Keyword::While) => {
                Ok(Self::WhileLoop(WhileLoop::parse(cur_token, reader)?))
            }
            Token::Keyword(Keyword::Do) => {
                Ok(Self::DoWhileLoop(DoWhileLoop::parse(cur_token, reader)?))
            }
            Token::Keyword(Keyword::Return) => Ok(Self::ReturnStatement(ReturnStatement::parse(
                cur_token, reader,
            )?)),
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

impl Expression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<Vec<llvm_ast::Expression>, precompiler::Error<Identifier>> {
        match self {
            Self::FunctionDeclaration(function_declaration) => {
                let function_declaration = function_declaration.precompile(precompiler)?;
                precompiler.function_declarations.push(function_declaration);
                Ok(Vec::new())
            }
            Self::FunctionCall(function_call) => Ok(vec![llvm_ast::Expression::FunctionCall(
                function_call.precompile(precompiler)?,
            )]),
            Self::VariableDeclaration(variable_declaration) => {
                Ok(vec![llvm_ast::Expression::VariableDeclaration(
                    variable_declaration.precompile(precompiler)?,
                )])
            }
            Self::VariableAssigment(variable_assigment) => {
                Ok(vec![llvm_ast::Expression::VariableAssigment(
                    variable_assigment.precompile(precompiler)?,
                )])
            }
            Self::ReturnStatement(return_statement) => {
                Ok(vec![llvm_ast::Expression::ReturnStatement(
                    return_statement.precompile(precompiler)?,
                )])
            }
            Self::BlockStatement(block_statement) => block_statement.precompile(precompiler),
            Self::IfElseStatement(if_else_statement) => {
                Ok(vec![llvm_ast::Expression::IfElseStatement(
                    if_else_statement.precompile(precompiler)?,
                )])
            }
            Self::WhileLoop(while_loop) => Ok(vec![llvm_ast::Expression::WhileLoop(
                while_loop.precompile(precompiler)?,
            )]),
            Self::DoWhileLoop(do_while_loop) => Ok(vec![llvm_ast::Expression::DoWhileLoop(
                do_while_loop.precompile(precompiler)?,
            )]),
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
                    right: Some(VariableExpression::VariableValue(VariableValue::Number(
                        12_f64
                    )))
                }
            )))
        );
    }

    #[test]
    fn parse_expression_test2() {
        let mut reader = TokenReader::new("let name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableDeclaration(VariableDeclaration(
                VariableAssigment {
                    left: "name".to_string().into(),
                    right: Some(VariableExpression::VariableValue(VariableValue::Number(
                        12_f64
                    )))
                }
            )))
        );
    }

    #[test]
    fn parse_expression_test3() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableAssigment(VariableAssigment {
                left: "name".to_string().into(),
                right: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            })
        );
    }

    #[test]
    fn parse_expression_test4() {
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
                    right: Some(VariableExpression::VariableValue(
                        VariableValue::Identifier("name2".to_string().into())
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
                        right: Some(VariableExpression::VariableValue(
                            VariableValue::Identifier("name2".to_string().into())
                        ))
                    }),
                    Expression::BlockStatement(BlockStatement {
                        body: vec![
                            Expression::VariableAssigment(VariableAssigment {
                                left: "name1".to_string().into(),
                                right: Some(VariableExpression::VariableValue(
                                    VariableValue::Identifier("name2".to_string().into())
                                ))
                            }),
                            Expression::VariableAssigment(VariableAssigment {
                                left: "name1".to_string().into(),
                                right: Some(VariableExpression::VariableValue(
                                    VariableValue::Identifier("name2".to_string().into())
                                ))
                            }),
                        ]
                    })
                ]
            })
        );
    }

    #[test]
    fn parse_expression_test5() {
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
                right: Some(VariableExpression::VariableValue(VariableValue::Number(
                    6_f64
                )))
            }))
        );
    }
}
