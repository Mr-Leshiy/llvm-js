use super::{Identifier, LogicalExpression};
use crate::{
    llvm_ast,
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Literal as LiteralToken, Logical, Parser, Token, TokenReader};
use std::io::Read;

/// VariableValue
#[derive(Clone, Debug, PartialEq)]
pub enum VariableValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Identifier(Identifier),
    LogicalExpression(Box<LogicalExpression>),
}

impl VariableValue {
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
            Token::Logical(Logical::Not) => {
                Self::LogicalExpression(Box::new(LogicalExpression::Not(
                    VariableValue::parse_impl(reader.next_token()?, reader, true)?,
                )))
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
                        right: VariableValue::parse_impl(reader.next_token()?, reader, false)?,
                    })))
                }
                Token::Logical(Logical::And) => {
                    reader.reset_saving();
                    Ok(Self::LogicalExpression(Box::new(LogicalExpression::And {
                        left,
                        right: VariableValue::parse_impl(reader.next_token()?, reader, false)?,
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

impl Parser for VariableValue {
    fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        Self::parse_impl(cur_token, reader, false)
    }
}

impl Precompile for VariableValue {
    type Output = llvm_ast::VariableValue;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
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
            Self::LogicalExpression(_logical) => unimplemented!(),
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
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Boolean(true)),
        );

        let mut reader = TokenReader::new("false".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Boolean(false)),
        );

        let mut reader = TokenReader::new("12".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Number(12_f64)),
        );

        let mut reader = TokenReader::new(r#""name""#.as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::String("name".to_string())),
        );

        let mut reader = TokenReader::new("name".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Identifier(Identifier {
                name: "name".to_string()
            })),
        );

        let mut reader = TokenReader::new("!true".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableValue::Boolean(true))
            ))),
        );
    }

    #[test]
    fn parse_not_logical_expression_test() {
        let mut reader = TokenReader::new("!true".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableValue::Boolean(true))
            ))),
        );

        let mut reader = TokenReader::new("!false".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableValue::Boolean(false))
            ))),
        );

        let mut reader = TokenReader::new("!a".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Not(VariableValue::Identifier("a".to_string().into()))
            ))),
        );
    }

    #[test]
    fn parse_and_logical_expression_test() {
        let mut reader = TokenReader::new("true && false".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::And {
                    left: VariableValue::Boolean(true),
                    right: VariableValue::Boolean(false)
                }
            ))),
        );

        let mut reader = TokenReader::new("false && a".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::And {
                    left: VariableValue::Boolean(false),
                    right: VariableValue::Identifier("a".to_string().into())
                }
            ))),
        );

        let mut reader = TokenReader::new("a && b".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::And {
                    left: VariableValue::Identifier("a".to_string().into()),
                    right: VariableValue::Identifier("b".to_string().into())
                }
            ))),
        );
    }

    #[test]
    fn parse_or_logical_expression_test() {
        let mut reader = TokenReader::new("true || false".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableValue::Boolean(true),
                    right: VariableValue::Boolean(false)
                }
            ))),
        );

        let mut reader = TokenReader::new("false || a".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableValue::Boolean(false),
                    right: VariableValue::Identifier("a".to_string().into())
                }
            ))),
        );

        let mut reader = TokenReader::new("a || b".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableValue::Identifier("a".to_string().into()),
                    right: VariableValue::Identifier("b".to_string().into())
                }
            ))),
        );
    }

    #[test]
    fn parse_logical_expression_test_1() {
        let mut reader = TokenReader::new("!a || b && !c".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::LogicalExpression(Box::new(
                LogicalExpression::Or {
                    left: VariableValue::LogicalExpression(Box::new(LogicalExpression::Not(
                        VariableValue::Identifier("a".to_string().into())
                    ))),
                    right: VariableValue::LogicalExpression(Box::new(LogicalExpression::And {
                        left: VariableValue::Identifier("b".to_string().into()),
                        right: VariableValue::LogicalExpression(Box::new(LogicalExpression::Not(
                            VariableValue::Identifier("c".to_string().into())
                        )))
                    }))
                }
            ))),
        );
    }
}
