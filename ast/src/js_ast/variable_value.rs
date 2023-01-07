use super::{ArrayExpression, MemberExpression, ObjectExpression};
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Arithmetic, Literal, Separator, Token, TokenReader};
use std::io::Read;

/// VariableValue
#[derive(Clone, Debug, PartialEq)]
pub enum VariableValue {
    Undefined,
    Null,
    NaN,
    Infinity,
    NegInfinity,
    Boolean(bool),
    Number(f64),
    String(String),
    MemberExpression(MemberExpression),
    ObjectExpression(ObjectExpression),
    ArrayExpression(ArrayExpression),
}

impl VariableValue {
    pub fn parse<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        match cur_token {
            Token::Literal(Literal::Undefined) => Ok(Self::Undefined),
            Token::Literal(Literal::Null) => Ok(Self::Null),
            Token::Literal(Literal::NaN) => Ok(Self::NaN),
            Token::Literal(Literal::Infinity) => Ok(Self::Infinity),
            Token::Literal(Literal::Boolean(boolean)) => Ok(Self::Boolean(boolean)),
            Token::Literal(Literal::Number(val)) => Ok(Self::Number(val)),
            Token::Literal(Literal::String(val)) => Ok(Self::String(val)),
            Token::Ident(_) => Ok(Self::MemberExpression(MemberExpression::parse(
                cur_token, reader,
            )?)),
            // negative
            Token::Arithmetic(Arithmetic::Sub) => match reader.next_token()? {
                Token::Literal(Literal::Infinity) => Ok(Self::NegInfinity),
                Token::Literal(Literal::Number(val)) => Ok(Self::Number(-val)),
                token => Err(LexerError::UnexpectedToken(token)),
            },
            Token::Separator(Separator::OpenCurlyBrace) => Ok(Self::ObjectExpression(
                ObjectExpression::parse(cur_token, reader)?,
            )),
            Token::Separator(Separator::OpenSquareBracket) => Ok(Self::ArrayExpression(
                ArrayExpression::parse(cur_token, reader)?,
            )),
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }
}

impl VariableValue {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::VariableValue, PrecompilerError> {
        match self {
            Self::Undefined => Ok(llvm_ast::VariableValue::Undefined),
            Self::Null => Ok(llvm_ast::VariableValue::Null),
            Self::NaN => Ok(llvm_ast::VariableValue::NaN),
            Self::Infinity => Ok(llvm_ast::VariableValue::Infinity),
            Self::NegInfinity => Ok(llvm_ast::VariableValue::NegInfinity),
            Self::Boolean(boolean) => Ok(llvm_ast::VariableValue::Boolean(boolean)),
            Self::MemberExpression(member_expression) => {
                Ok(llvm_ast::VariableValue::MemberExpression(
                    member_expression.precompile(precompiler)?,
                ))
            }
            Self::Number(number) => Ok(llvm_ast::VariableValue::FloatNumber(number)),
            Self::String(string) => Ok(llvm_ast::VariableValue::String(string)),
            Self::ObjectExpression(object_expression) => {
                Ok(llvm_ast::VariableValue::ObjectExpression(
                    object_expression.precompile(precompiler)?,
                ))
            }
            Self::ArrayExpression(array_expression) => Ok(
                llvm_ast::VariableValue::ArrayExpression(array_expression.precompile(precompiler)?),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_variable_value_test() {
        let mut reader = TokenReader::new("undefined".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Undefined),
        );

        let mut reader = TokenReader::new("null".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Null),
        );

        let mut reader = TokenReader::new("NaN".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::NaN),
        );

        let mut reader = TokenReader::new("Infinity".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Infinity),
        );

        let mut reader = TokenReader::new("-Infinity".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::NegInfinity),
        );

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

        let mut reader = TokenReader::new("-12".as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::Number(-12_f64)),
        );

        let mut reader = TokenReader::new(r#""name""#.as_bytes());
        assert_eq!(
            VariableValue::parse(reader.next_token().unwrap(), &mut reader),
            Ok(VariableValue::String("name".to_string())),
        );
    }
}
