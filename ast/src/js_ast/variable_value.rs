use super::Identifier;
use crate::{
    llvm_ast,
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Literal as LiteralToken, Parser, Token, TokenReader};
use std::io::Read;

/// VariableValue
#[derive(Clone, Debug, PartialEq)]
pub enum VariableValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Identifier(Identifier),
}

impl Parser for VariableValue {
    fn parse<R: Read>(cur_token: Token, _: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        match cur_token {
            Token::Literal(LiteralToken::Boolean(boolean)) => Ok(Self::Boolean(boolean)),
            Token::Literal(LiteralToken::Number(val)) => Ok(Self::Number(val)),
            Token::Literal(LiteralToken::String(val)) => Ok(Self::String(val)),
            Token::Ident(name) => Ok(Self::Identifier(Identifier { name })),
            token => Err(lexer::Error::UnexpectedToken(token)),
        }
    }
}

impl Precompile for VariableValue {
    type Output = llvm_ast::VariableValue;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        match self {
            VariableValue::Boolean(boolean) => Ok(Self::Output::Boolean(boolean)),
            VariableValue::Identifier(identifier) => match precompiler.variables.get(&identifier) {
                Some(index) => Ok(Self::Output::Identifier(llvm_ast::VariableName::new(
                    identifier.name,
                    index,
                ))),
                None => Err(precompiler::Error::UndefinedVariable(identifier.clone())),
            },
            VariableValue::Number(number) => Ok(Self::Output::FloatNumber(number)),
            VariableValue::String(string) => Ok(Self::Output::String(string)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value_test() {
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
    }
}
