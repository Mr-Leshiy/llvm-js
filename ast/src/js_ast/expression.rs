use super::{
    return_statement::ReturnStatement, BlockStatement, DoWhileLoop, FunctionDeclaration,
    IfElseStatement, VariableAssigment, VariableDeclaration, VariableExpression, WhileLoop,
};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Keyword, Separator, Token, TokenReader};
use std::{fmt::Debug, io::Read};

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
    VariableExpression(VariableExpression),
    BlockStatement(BlockStatement),
    IfElseStatement(IfElseStatement),
    WhileLoop(WhileLoop),
    DoWhileLoop(DoWhileLoop),
    ReturnStatement(ReturnStatement),
}

impl Expression {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        match cur_token {
            Token::Keyword(Keyword::Function) => Ok(Self::FunctionDeclaration(
                FunctionDeclaration::parse(cur_token, reader)?,
            )),
            Token::Keyword(Keyword::Var | Keyword::Let) => Ok(Self::VariableDeclaration(
                VariableDeclaration::parse(cur_token, reader)?,
            )),
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
            cur_token => {
                reader.start_saving();
                if let Ok(res) = VariableAssigment::parse(cur_token.clone(), reader) {
                    reader.reset_saving();
                    Ok(Self::VariableAssigment(res))
                } else {
                    reader.stop_saving();
                    Ok(Self::VariableExpression(VariableExpression::parse(
                        cur_token, reader,
                    )?))
                }
            }
        }
    }
}

impl Expression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<Vec<llvm_ast::Expression>, PrecompilerError> {
        match self {
            Self::FunctionDeclaration(function_declaration) => {
                let (function_declaration, variable_function_declaration) =
                    function_declaration.precompile(precompiler)?;
                precompiler.insert_function_declaration(function_declaration);
                Ok(vec![llvm_ast::Expression::VariableFunctionDeclaration(
                    variable_function_declaration,
                )])
            }
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
            Self::VariableExpression(variable_expression) => {
                Ok(vec![llvm_ast::Expression::VariableExpression(
                    variable_expression.precompile(precompiler)?,
                )])
            }
            Self::ReturnStatement(return_statement) => {
                Ok(vec![llvm_ast::Expression::ReturnStatement(
                    return_statement.precompile(precompiler)?,
                )])
            }
            Self::BlockStatement(block_statement) => Ok(block_statement.precompile(precompiler)?),
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
    use crate::js_ast::{
        member_expression::PropertyType, BinaryExpType, BinaryExpression, FunctionCall,
        MemberExpression, Property, VariableExpression, VariableValue,
    };

    #[test]
    fn parse_expression_test1() {
        let mut reader = TokenReader::new("var name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableDeclaration(VariableDeclaration {
                name: "name".to_string().into(),
                value: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            }))
        );
    }

    #[test]
    fn parse_expression_test2() {
        let mut reader = TokenReader::new("let name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableDeclaration(VariableDeclaration {
                name: "name".to_string().into(),
                value: Some(VariableExpression::VariableValue(VariableValue::Number(
                    12_f64
                )))
            }))
        );
    }

    #[test]
    fn parse_expression_test3() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableAssigment(VariableAssigment {
                left: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name".to_string().into()
                )),
                right: VariableExpression::VariableValue(VariableValue::Number(12_f64))
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
                    left: VariableExpression::VariableValue(VariableValue::Identifier(
                        "name1".to_string().into()
                    )),
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
                        left: VariableExpression::VariableValue(VariableValue::Identifier(
                            "name1".to_string().into()
                        )),
                        right: VariableExpression::VariableValue(VariableValue::Identifier(
                            "name2".to_string().into()
                        ))
                    }),
                    Expression::BlockStatement(BlockStatement {
                        body: vec![
                            Expression::VariableAssigment(VariableAssigment {
                                left: VariableExpression::VariableValue(VariableValue::Identifier(
                                    "name1".to_string().into()
                                )),
                                right: VariableExpression::VariableValue(
                                    VariableValue::Identifier("name2".to_string().into())
                                )
                            }),
                            Expression::VariableAssigment(VariableAssigment {
                                left: VariableExpression::VariableValue(VariableValue::Identifier(
                                    "name1".to_string().into()
                                )),
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
    fn parse_expression_test5() {
        let mut reader = TokenReader::new("1; (1 + 2); foo(a, b); a = 6;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableExpression(VariableExpression::VariableValue(
                VariableValue::Number(1_f64)
            ))
        );
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableExpression(VariableExpression::BinaryExpression(
                BinaryExpression {
                    left: VariableExpression::VariableValue(VariableValue::Number(1_f64)),
                    right: VariableExpression::VariableValue(VariableValue::Number(2_f64)),
                    exp_type: BinaryExpType::Add,
                }
                .into()
            ))
        );
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableExpression(VariableExpression::FunctionCall(FunctionCall {
                name: "foo".to_string().into(),
                args: vec![
                    VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into()
                    ))
                ]
            }))
        );
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableAssigment(VariableAssigment {
                left: VariableExpression::VariableValue(VariableValue::Identifier(
                    "a".to_string().into()
                )),
                right: VariableExpression::VariableValue(VariableValue::Number(6_f64))
            }))
        );
    }

    #[test]
    fn parse_expression_test6() {
        let mut reader = TokenReader::new("name.name()".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableExpression(VariableExpression::VariableValue(
                VariableValue::MemberExpression(MemberExpression {
                    variable_name: "name".to_string().into(),
                    property: Property {
                        object: PropertyType::FunctionCall(FunctionCall {
                            name: "name".to_string().into(),
                            args: vec![]
                        }),
                        property: None
                    }
                    .into()
                })
            ))
        );
    }

    #[test]
    fn parse_expression_test7() {
        let mut reader = TokenReader::new("(1 + 2) = name2;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableAssigment(VariableAssigment {
                left: VariableExpression::BinaryExpression(
                    BinaryExpression {
                        left: VariableExpression::VariableValue(VariableValue::Number(1_f64)),
                        right: VariableExpression::VariableValue(VariableValue::Number(2_f64)),
                        exp_type: BinaryExpType::Add,
                    }
                    .into()
                ),
                right: VariableExpression::VariableValue(VariableValue::Identifier(
                    "name2".to_string().into()
                ))
            }))
        );
    }
}
