use super::{Identifier, VariableValue};
use crate::{
    llvm_ast,
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Parser, Separator, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<VariableValue>,
}

impl Parser for FunctionCall {
    fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, lexer::Error> {
        // parse function name
        let name = Identifier::parse(cur_token, reader)?;

        // parse function args
        let args = match reader.next_token()? {
            Token::Separator(Separator::OpenBrace) => {
                let mut args = Vec::new();
                cur_token = reader.next_token()?;
                loop {
                    let arg = match cur_token {
                        Token::Separator(Separator::CloseBrace) => break,
                        cur_token => VariableValue::parse(cur_token, reader)?,
                    };
                    args.push(arg);

                    cur_token = match reader.next_token()? {
                        Token::Separator(Separator::CloseBrace) => break,
                        Token::Separator(Separator::Comma) => reader.next_token()?,
                        token => return Err(lexer::Error::UnexpectedToken(token)),
                    };
                }
                Ok(args)
            }
            token => Err(lexer::Error::UnexpectedToken(token)),
        }?;
        Ok(Self { name, args })
    }
}

impl Precompile for FunctionCall {
    type Output = llvm_ast::FunctionCall;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        match precompiler.functions.get(&self.name) {
            Some(_index) => {
                // check if arguments exist
                let mut args = Vec::new();
                for arg in self.args {
                    args.push(arg.precompile(precompiler)?);
                }

                Ok(llvm_ast::FunctionCall {
                    name: self.name.name,
                    args,
                })
            }
            None => Err(precompiler::Error::UndefinedFunction(self.name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_function_call_test() {
        let mut reader = TokenReader::new(r#"foo(a, b, "val", 5)"#.as_bytes());
        assert_eq!(
            FunctionCall::parse(reader.next_token().unwrap(), &mut reader),
            Ok(FunctionCall {
                name: "foo".to_string().into(),
                args: vec![
                    VariableValue::Identifier("a".to_string().into()),
                    VariableValue::Identifier("b".to_string().into()),
                    VariableValue::String("val".to_string()),
                    VariableValue::Number(5_f64),
                ]
            })
        );
    }

    #[test]
    fn precompile_function_call_test() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler.functions.insert("name_1".to_string().into());
        precompiler.variables.insert("a".to_string().into());
        precompiler.variables.insert("b".to_string().into());

        let function_call = FunctionCall {
            name: "name_1".to_string().into(),

            args: vec![
                VariableValue::Identifier("a".to_string().into()),
                VariableValue::Identifier("b".to_string().into()),
                VariableValue::String("val".to_string()),
                VariableValue::Number(5_f64),
            ],
        };

        assert_eq!(
            function_call.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionCall {
                name: "name_1".to_string(),
                args: vec![
                    llvm_ast::VariableValue::Identifier(llvm_ast::VariableName::new(
                        "a".to_string(),
                        0
                    )),
                    llvm_ast::VariableValue::Identifier(llvm_ast::VariableName::new(
                        "b".to_string(),
                        0
                    )),
                    llvm_ast::VariableValue::String("val".to_string()),
                    llvm_ast::VariableValue::FloatNumber(5_f64),
                ],
            })
        );
    }

    #[test]
    fn precompile_function_call_error() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());

        let function_call = FunctionCall {
            name: "name_1".to_string().into(),

            args: vec![],
        };

        assert_eq!(
            function_call.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedFunction(
                "name_1".to_string().into()
            ))
        );
    }
}
