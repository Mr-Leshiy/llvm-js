use super::{Identifier, LogicalExpression};
use crate::llvm_ast;
use lexer::{Literal as LiteralToken, Logical, Parser, Separator, Token, TokenReader};
use precompiler::{self, Precompile, Precompiler};
use std::io::Read;

/// VariableValue
#[derive(Clone, Debug, PartialEq)]
pub enum VariableExpression {
    Boolean(bool),
    Number(f64),
    String(String),
    Identifier(Identifier),
    LogicalExpression(Box<LogicalExpression>),
    Grouping(Box<VariableExpression>),
}

impl VariableExpression {
    fn parse_impl<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
        is_unary: bool,
    ) -> Result<Self, lexer::Error> {
        let left = match cur_token {
            Token::Literal(LiteralToken::Boolean(boolean)) => Self::Boolean(boolean),
            Token::Literal(LiteralToken::Number(val)) => Self::Number(val),
            Token::Literal(LiteralToken::String(val)) => Self::String(val),
            Token::Ident(name) => Self::Identifier(Identifier { name }),
            Token::Logical(Logical::Not) => Self::LogicalExpression(Box::new(
                LogicalExpression::Not(Self::parse_impl(reader.next_token()?, reader, true)?),
            )),
            Token::Separator(Separator::OpenBrace) => {
                let value = Self::parse_impl(reader.next_token()?, reader, false)?;
                match reader.next_token()? {
                    Token::Separator(Separator::CloseBrace) => Self::Grouping(Box::new(value)),
                    token => return Err(lexer::Error::UnexpectedToken(token)),
                }
            }
            token => return Err(lexer::Error::UnexpectedToken(token)),
        };
        if is_unary {
            Ok(left)
        } else {
            reader.start_saving();
            match reader.next_token()? {
                Token::Logical(Logical::Or) => {
                    reader.reset_saving();
                    Ok(Self::LogicalExpression(Box::new(LogicalExpression::Or {
                        left,
                        right: VariableExpression::parse_impl(reader.next_token()?, reader, false)?,
                    })))
                }
                Token::Logical(Logical::And) => {
                    reader.reset_saving();
                    Ok(Self::LogicalExpression(Box::new(LogicalExpression::And {
                        left,
                        right: VariableExpression::parse_impl(reader.next_token()?, reader, false)?,
                    })))
                }
                _ => {
                    reader.stop_saving();
                    Ok(left)
                }
            }
        }
    }
}

impl Parser for VariableExpression {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        Self::parse_impl(cur_token, reader, false)
    }
}

impl Precompile<Identifier, llvm_ast::FunctionDeclaration> for VariableExpression {
    type Output = llvm_ast::VariableValue;
    fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<Self::Output, precompiler::Error<Identifier>> {
        match self {
            Self::Boolean(boolean) => Ok(Self::Output::Boolean(boolean)),
            Self::Identifier(identifier) => match precompiler.variables.get(&identifier) {
                Some(index) => Ok(Self::Output::Identifier(llvm_ast::Identifier::new(
                    identifier.name,
                    index,
                ))),
                None => Err(precompiler::Error::UndefinedVariable(identifier.clone())),
            },
            Self::Number(number) => Ok(Self::Output::FloatNumber(number)),
            Self::String(string) => Ok(Self::Output::String(string)),
            Self::LogicalExpression(_logical) => todo!("implement"),
            Self::Grouping(_grouping) => todo!("implement"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_variable_value_test() {
        let mut reader = TokenReader::new("true".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Boolean(true)),
        );

        let mut reader = TokenReader::new("false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Boolean(false)),
        );

        let mut reader = TokenReader::new("12".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Number(12_f64)),
        );

        let mut reader = TokenReader::new(r#""name""#.as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::String("name".to_string())),
        );

        let mut reader = TokenReader::new("name".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Identifier(Identifier {
                name: "name".to_string()
            })),
        );

        let mut reader = TokenReader::new("!true".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableExpression::Boolean(true))
            ))),
        );
    }

    #[test]
    fn parse_not_logical_expression_test() {
        let mut reader = TokenReader::new("!true".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableExpression::Boolean(true))
            ))),
        );

        let mut reader = TokenReader::new("!false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableExpression::Boolean(false))
            ))),
        );

        let mut reader = TokenReader::new("!a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableExpression::Identifier("a".to_string().into()))
            ))),
        );
    }

    #[test]
    fn parse_and_logical_expression_test() {
        let mut reader = TokenReader::new("true && false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::And {
                    left: VariableExpression::Boolean(true),
                    right: VariableExpression::Boolean(false)
                }
            ))),
        );

        let mut reader = TokenReader::new("false && a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::And {
                    left: VariableExpression::Boolean(false),
                    right: VariableExpression::Identifier("a".to_string().into())
                }
            ))),
        );

        let mut reader = TokenReader::new("a && b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::And {
                    left: VariableExpression::Identifier("a".to_string().into()),
                    right: VariableExpression::Identifier("b".to_string().into())
                }
            ))),
        );
    }

    #[test]
    fn parse_or_logical_expression_test() {
        let mut reader = TokenReader::new("true || false".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableExpression::Boolean(true),
                    right: VariableExpression::Boolean(false)
                }
            ))),
        );

        let mut reader = TokenReader::new("false || a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableExpression::Boolean(false),
                    right: VariableExpression::Identifier("a".to_string().into())
                }
            ))),
        );

        let mut reader = TokenReader::new("a || b".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableExpression::Identifier("a".to_string().into()),
                    right: VariableExpression::Identifier("b".to_string().into())
                }
            ))),
        );
    }

    #[test]
    fn parse_logical_expression_test_1() {
        let mut reader = TokenReader::new("!a || b && !c".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableExpression::LogicalExpression(Box::new(LogicalExpression::Not(
                        VariableExpression::Identifier("a".to_string().into())
                    ))),
                    right: VariableExpression::LogicalExpression(Box::new(
                        LogicalExpression::And {
                            left: VariableExpression::Identifier("b".to_string().into()),
                            right: VariableExpression::LogicalExpression(Box::new(
                                LogicalExpression::Not(VariableExpression::Identifier(
                                    "c".to_string().into()
                                ))
                            ))
                        }
                    ))
                }
            ))),
        );

        let mut reader = TokenReader::new("!!!!a".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableExpression::LogicalExpression(Box::new(
                    LogicalExpression::Not(VariableExpression::LogicalExpression(Box::new(
                        LogicalExpression::Not(VariableExpression::LogicalExpression(Box::new(
                            LogicalExpression::Not(VariableExpression::Identifier(
                                "a".to_string().into()
                            ))
                        )))
                    )))
                )))
            )))
        );
    }

    #[test]
    fn parse_grouping_test() {
        let mut reader = TokenReader::new("(!a || (b && !c))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Grouping(Box::new(
                VariableExpression::LogicalExpression(Box::new(LogicalExpression::Or {
                    left: VariableExpression::LogicalExpression(Box::new(LogicalExpression::Not(
                        VariableExpression::Identifier("a".to_string().into())
                    ))),
                    right: VariableExpression::Grouping(Box::new(
                        VariableExpression::LogicalExpression(Box::new(LogicalExpression::And {
                            left: VariableExpression::Identifier("b".to_string().into()),
                            right: VariableExpression::LogicalExpression(Box::new(
                                LogicalExpression::Not(VariableExpression::Identifier(
                                    "c".to_string().into()
                                ))
                            ))
                        }))
                    ))
                }))
            )))
        );

        let mut reader = TokenReader::new("(!(a))".as_bytes());
        assert_eq!(
            VariableExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableExpression::Grouping(Box::new(
                VariableExpression::LogicalExpression(Box::new(LogicalExpression::Not(
                    VariableExpression::Grouping(Box::new(VariableExpression::Identifier(
                        "a".to_string().into()
                    )))
                )))
            )))
        );
    }
}
