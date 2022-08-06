use super::{Expression, Identifier};
use crate::llvm_ast;
use lexer::{Parser, Separator, Token, TokenReader};
use precompiler::{self, Precompile, Precompiler};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct BlockStatement {
    pub body: Vec<Expression>,
}

impl Parser for BlockStatement {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        match cur_token {
            Token::Separator(Separator::OpenCurlyBrace) => {
                let mut body = Vec::new();
                let mut cur_token = reader.next_token()?;
                loop {
                    let expr = match cur_token {
                        Token::Separator(Separator::CloseCurlyBrace) => break,
                        cur_token => Expression::parse(cur_token, reader)?,
                    };

                    cur_token = reader.next_token()?;
                    body.push(expr);
                }

                Ok(Self { body })
            }
            token => Err(lexer::Error::UnexpectedToken(token)),
        }
    }
}

impl Precompile<Identifier, llvm_ast::FunctionDeclaration> for BlockStatement {
    type Output = Vec<llvm_ast::Expression>;
    fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<Self::Output, precompiler::Error<Identifier>> {
        let mut res = Vec::with_capacity(self.body.len());
        let variables_len = precompiler.variables.len();
        let functions_len = precompiler.functions.len();
        for expr in self.body {
            expr.precompile(precompiler)?
                .into_iter()
                .for_each(|expr| res.push(expr))
        }
        precompiler
            .variables
            .remove_last_added(precompiler.variables.len() - variables_len);
        precompiler
            .functions
            .remove_last_added(precompiler.functions.len() - functions_len);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{
        FunctionDeclaration, VariableAssigment, VariableDeclaration, VariableExpression,
        VariableValue,
    };

    #[test]
    fn parse_block_statement_test() {
        let mut reader = TokenReader::new("{ }".as_bytes());
        assert_eq!(
            BlockStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(BlockStatement { body: vec![] })
        );

        let mut reader = TokenReader::new("{ name1 = name2; }".as_bytes());
        assert_eq!(
            BlockStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(BlockStatement {
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
            BlockStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(BlockStatement {
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
    fn precompile_block_statement_test_1() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        assert_eq!(precompiler.variables.len(), 0);
        let block_statement = BlockStatement {
            body: vec![Expression::VariableDeclaration(VariableDeclaration(
                VariableAssigment {
                    left: "name_1".to_string().into(),
                    right: VariableExpression::VariableValue(VariableValue::Number(64_f64)),
                },
            ))],
        };

        assert_eq!(
            block_statement.precompile(&mut precompiler),
            Ok(vec![llvm_ast::Expression::VariableDeclaration(
                llvm_ast::VariableDeclaration(llvm_ast::VariableAssigment {
                    name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                    value: llvm_ast::VariableValue::FloatNumber(64_f64),
                })
            )])
        );
        assert_eq!(precompiler.variables.len(), 0);
    }

    #[test]
    fn precompile_block_statement_test_2() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        assert_eq!(precompiler.functions.len(), 0);
        let block_statement = BlockStatement {
            body: vec![Expression::FunctionDeclaration(FunctionDeclaration {
                name: "name_1".to_string().into(),
                args: vec![],
                body: BlockStatement { body: vec![] },
            })],
        };

        assert_eq!(block_statement.precompile(&mut precompiler), Ok(vec![]));
        assert_eq!(precompiler.functions.len(), 0);
        assert_eq!(precompiler.function_declarations.len(), 1);
    }
}
