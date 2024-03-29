use super::VariableExpression;
use crate::{llvm_ast, LexerError, Precompiler, PrecompilerError};
use lexer::{Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayExpression {
    pub values: Vec<VariableExpression>,
}

impl ArrayExpression {
    pub fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, LexerError> {
        match cur_token {
            Token::Separator(Separator::OpenSquareBracket) => {
                let mut values = Vec::new();
                cur_token = reader.next_token()?;
                loop {
                    let arg = match cur_token {
                        Token::Separator(Separator::CloseSquareBracket) => break,
                        cur_token => VariableExpression::parse(cur_token, reader)?,
                    };
                    values.push(arg);

                    cur_token = match reader.next_token()? {
                        Token::Separator(Separator::CloseSquareBracket) => break,
                        Token::Separator(Separator::Comma) => reader.next_token()?,
                        token => return Err(LexerError::UnexpectedToken(token)),
                    };
                }

                Ok(Self { values })
            }
            token => Err(LexerError::UnexpectedToken(token)),
        }
    }

    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::ArrayExpression, PrecompilerError> {
        let mut values = Vec::new();
        for value in self.values {
            values.push(value.precompile(precompiler)?);
        }
        Ok(llvm_ast::ArrayExpression { values })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::VariableValue;

    #[test]
    fn parse_array_expression_test() {
        let mut reader = TokenReader::new("[]".as_bytes());
        assert_eq!(
            ArrayExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(ArrayExpression { values: vec![] })
        );

        let mut reader = TokenReader::new(r#"[1, "name", true, name]"#.as_bytes());
        assert_eq!(
            ArrayExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(ArrayExpression {
                values: vec![
                    VariableExpression::VariableValue(VariableValue::Number(1_f64)),
                    VariableExpression::VariableValue(VariableValue::String("name".to_string())),
                    VariableExpression::VariableValue(VariableValue::Boolean(true)),
                    VariableExpression::VariableValue(VariableValue::Identifier(
                        "name".to_string().into()
                    )),
                ]
            })
        );
    }
}
