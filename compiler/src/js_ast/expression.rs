use super::{
    BlockStatement, FunctionCall, FunctionDeclaration, VariableAssigment, VariableDeclaration,
};
use crate::{
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Keyword, Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    FunctionDeclaration(FunctionDeclaration),
    FunctionCall(FunctionCall),
    VariableDeclaration(VariableDeclaration),
    VariableAssigment(VariableAssigment),
    BlockStatement(BlockStatement),
}

impl Parser for Expression {
    fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, parser::Error> {
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
                    Ok(res) => Ok(Self::FunctionCall(res)),
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
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

impl Precompile for Expression {
    type Output = Vec<llvm_ast::Expression>;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
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
    use crate::js_ast::{Identifier, Literal, RightAssigmentValue};

    #[test]
    fn parse_expression_variable_declaration_test() {
        let mut reader = TokenReader::new("var name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(Expression::VariableDeclaration(VariableDeclaration(
                VariableAssigment {
                    left: Identifier {
                        name: "name".to_string()
                    },
                    right: RightAssigmentValue::Literal(Literal::Number(12_f64))
                }
            )))
        );
    }

    #[test]
    fn expression_assigment_test() {
        let mut reader = TokenReader::new("name = 12;".as_bytes());
        assert_eq!(
            Expression::parse(reader.next_token().unwrap(), &mut reader).unwrap(),
            Expression::VariableAssigment(VariableAssigment {
                left: Identifier {
                    name: "name".to_string()
                },
                right: RightAssigmentValue::Literal(Literal::Number(12_f64))
            })
        );
    }

    #[test]
    fn expression_block_statement_test() {
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
                    left: Identifier {
                        name: "name1".to_string()
                    },
                    right: RightAssigmentValue::Identifier(Identifier {
                        name: "name2".to_string()
                    })
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
                        left: Identifier {
                            name: "name1".to_string()
                        },
                        right: RightAssigmentValue::Identifier(Identifier {
                            name: "name2".to_string()
                        })
                    }),
                    Expression::BlockStatement(BlockStatement {
                        body: vec![
                            Expression::VariableAssigment(VariableAssigment {
                                left: Identifier {
                                    name: "name1".to_string()
                                },
                                right: RightAssigmentValue::Identifier(Identifier {
                                    name: "name2".to_string()
                                })
                            }),
                            Expression::VariableAssigment(VariableAssigment {
                                left: Identifier {
                                    name: "name1".to_string()
                                },
                                right: RightAssigmentValue::Identifier(Identifier {
                                    name: "name2".to_string()
                                })
                            }),
                        ]
                    })
                ]
            })
        );
    }
}
