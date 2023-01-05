use super::{Identifier, VariableExpression};
use crate::{llvm_ast, Error, Precompiler, PrecompilerError};
use lexer::{Separator, Token, TokenReader};
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

        match reader.next_token()? {
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
                Ok(Self { name, args })
            }
            token => Err(Error::UnexpectedToken(token)),
        }
    }
}

impl FunctionCall {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::FunctionCall, PrecompilerError> {
        let index = precompiler.get_function(self.name.clone())?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{MemberExpression, VariableValue};

    #[test]
    fn parse_function_call_test() {
        let mut reader = TokenReader::new(r#"foo(a, b, "val", 5)"#.as_bytes());
        assert_eq!(
            FunctionCall::parse(reader.next_token().unwrap(), &mut reader),
            Ok(FunctionCall {
                name: "foo".to_string().into(),
                args: vec![
                    VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "a".to_string().into(),
                            property: None
                        }
                    )),
                    VariableExpression::VariableValue(VariableValue::MemberExpression(
                        MemberExpression {
                            variable_name: "b".to_string().into(),
                            property: None
                        }
                    )),
                    VariableExpression::VariableValue(VariableValue::String("val".to_string())),
                    VariableExpression::VariableValue(VariableValue::Number(5_f64)),
                ]
            })
        );
    }

    #[test]
    fn precompile_function_call_test() {
        let mut precompiler = Precompiler::new(std::iter::empty());
        precompiler.insert_function("name_1".to_string().into());
        precompiler.insert_variable("a".to_string().into());
        precompiler.insert_variable("b".to_string().into());

        let function_call = FunctionCall {
            name: "name_1".to_string().into(),

            args: vec![
                VariableExpression::VariableValue(VariableValue::MemberExpression(
                    MemberExpression {
                        variable_name: "a".to_string().into(),
                        property: None,
                    },
                )),
                VariableExpression::VariableValue(VariableValue::MemberExpression(
                    MemberExpression {
                        variable_name: "b".to_string().into(),
                        property: None,
                    },
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
                        llvm_ast::VariableValue::MemberExpression(llvm_ast::MemberExpression {
                            variable_name: llvm_ast::Identifier::new("a".to_string(), 0),
                            property: None
                        })
                    ),
                    llvm_ast::VariableExpression::VariableValue(
                        llvm_ast::VariableValue::MemberExpression(llvm_ast::MemberExpression {
                            variable_name: llvm_ast::Identifier::new("b".to_string(), 0),
                            property: None
                        })
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
        let mut precompiler = Precompiler::new(std::iter::empty());

        let function_call = FunctionCall {
            name: "name_1".to_string().into(),

            args: vec![],
        };

        assert_eq!(
            function_call.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedFunction("name_1".to_string().into()).into())
        );
    }
}
