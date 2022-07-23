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
    Number(f64),
    String(String),
    Identifier(Identifier),
}

impl Parser for VariableValue {
    fn parse<R: Read>(cur_token: Token, _: &mut TokenReader<R>) -> Result<Self, lexer::Error> {
        match cur_token {
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
        let res = match self {
            VariableValue::Identifier(identifier) => {
                precompiler
                    .variables
                    .contains(&identifier)
                    .then(|| ())
                    .ok_or_else(|| precompiler::Error::UndefinedVariable(identifier.clone()))?;
                Self::Output::Identifier(identifier.name)
            }
            VariableValue::Number(number) => Self::Output::FloatNumber(number),
            VariableValue::String(string) => Self::Output::String(string),
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value_test() {
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
