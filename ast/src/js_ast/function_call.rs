use super::{Identifier, VariableExpression};
use crate::{llvm_ast, Error};
use lexer::{Separator, Token, TokenReader};
use precompiler::{self, Precompiler};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<VariableExpression>,
}

impl FunctionCall {
    pub fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, Error> {
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
                        cur_token => VariableExpression::parse(cur_token, reader)?,
                    };
                    args.push(arg);

                    cur_token = match reader.next_token()? {
                        Token::Separator(Separator::CloseBrace) => break,
                        Token::Separator(Separator::Comma) => reader.next_token()?,
                        token => return Err(Error::UnexpectedToken(token)),
                    };
                }
                Ok(args)
            }
            token => Err(lexer::Error::UnexpectedToken(token)),
        }?;
        Ok(Self { name, args })
    }
}

impl FunctionCall {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::FunctionCall, precompiler::Error<Identifier>> {
        match precompiler.functions.get(&self.name) {
            Some(index) => {
                // check if arguments exist
                let mut args = Vec::new();
                for arg in self.args {
                    args.push(arg.precompile(precompiler)?);
                }

                Ok(llvm_ast::FunctionCall {
                    name: llvm_ast::Identifier::new(self.name.name, index),
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
    use crate::js_ast::VariableValue;

    #[test]
    fn parse_function_call_test() {
        let mut reader = TokenReader::new(r#"foo(a, b, "val", 5)"#.as_bytes());
        assert_eq!(
            FunctionCall::parse(reader.next_token().unwrap(), &mut reader),
            Ok(FunctionCall {
                name: "foo".to_string().into(),
                args: vec![
                    VariableExpression::VariableValue(VariableValue::Identifier(
                        "a".to_string().into()
                    )),
                    VariableExpression::VariableValue(VariableValue::Identifier(
                        "b".to_string().into()
                    )),
                    VariableExpression::VariableValue(VariableValue::String("val".to_string())),
                    VariableExpression::VariableValue(VariableValue::Number(5_f64)),
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
                VariableExpression::VariableValue(VariableValue::Identifier(
                    "a".to_string().into(),
                )),
                VariableExpression::VariableValue(VariableValue::Identifier(
                    "b".to_string().into(),
                )),
                VariableExpression::VariableValue(VariableValue::String("val".to_string())),
                VariableExpression::VariableValue(VariableValue::Number(5_f64)),
            ],
        };

        assert_eq!(
            function_call.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionCall {
                name: llvm_ast::Identifier::new("name_1".to_string(), 0),
                args: vec![
                    llvm_ast::VariableExpression::VariableValue(
                        llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                            "a".to_string(),
                            0
                        ))
                    ),
                    llvm_ast::VariableExpression::VariableValue(
                        llvm_ast::VariableValue::Identifier(llvm_ast::Identifier::new(
                            "b".to_string(),
                            0
                        ))
                    ),
                    llvm_ast::VariableExpression::VariableValue(llvm_ast::VariableValue::String(
                        "val".to_string()
                    )),
                    llvm_ast::VariableExpression::VariableValue(
                        llvm_ast::VariableValue::FloatNumber(5_f64)
                    ),
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
