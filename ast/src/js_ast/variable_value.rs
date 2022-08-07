use super::Identifier;
use crate::llvm_ast;
use lexer::{self, Literal, Parser, Token, TokenReader};
use precompiler::Precompiler;
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
            Token::Literal(Literal::Boolean(boolean)) => Ok(VariableValue::Boolean(boolean)),
            Token::Literal(Literal::Number(val)) => Ok(VariableValue::Number(val)),
            Token::Literal(Literal::String(val)) => Ok(VariableValue::String(val)),
            Token::Ident(name) => Ok(VariableValue::Identifier(Identifier { name })),
            token => Err(lexer::Error::UnexpectedToken(token)),
        }
    }
}

impl VariableValue {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::VariableValue, precompiler::Error<Identifier>> {
        match self {
            Self::Boolean(boolean) => Ok(llvm_ast::VariableValue::Boolean(boolean)),
            Self::Identifier(identifier) => match precompiler.variables.get(&identifier) {
                Some(index) => Ok(llvm_ast::VariableValue::Identifier(
                    llvm_ast::Identifier::new(identifier.name, index),
                )),
                None => Err(precompiler::Error::UndefinedVariable(identifier.clone())),
            },
            Self::Number(number) => Ok(llvm_ast::VariableValue::FloatNumber(number)),
            Self::String(string) => Ok(llvm_ast::VariableValue::String(string)),
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
    }
}
