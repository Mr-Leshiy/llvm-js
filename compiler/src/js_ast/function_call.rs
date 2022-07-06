use super::Identifier;
use crate::{
    lexer::{Separator, Token, TokenReader},
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<Identifier>,
}

impl Parser for FunctionCall {
    fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Self, parser::Error> {
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
                        cur_token => Identifier::parse(cur_token, reader)?,
                    };
                    args.push(arg);

                    cur_token = match reader.next_token()? {
                        Token::Separator(Separator::CloseBrace) => break,
                        Token::Separator(Separator::Comma) => reader.next_token()?,
                        token => return Err(parser::Error::UnexpectedToken(token)),
                    };
                }
                Ok(args)
            }
            token => Err(parser::Error::UnexpectedToken(token)),
        }?;
        Ok(Self { name, args })
    }
}

impl Precompile for FunctionCall {
    type Output = llvm_ast::FunctionCall;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        if precompiler.functions.contains(&self.name) {
            Ok(llvm_ast::FunctionCall {
                name: self.name.name,
            })
        } else {
            Err(precompiler::Error::UndefinedFunction(self.name))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_function_call_test() {
        let mut reader = TokenReader::new("foo(a, b)".as_bytes());
        assert_eq!(
            FunctionCall::parse(reader.next_token().unwrap(), &mut reader),
            Ok(FunctionCall {
                name: Identifier {
                    name: "foo".to_string()
                },
                args: vec![
                    Identifier {
                        name: "a".to_string()
                    },
                    Identifier {
                        name: "b".to_string()
                    }
                ]
            })
        );
    }

    #[test]
    fn precompile_function_call_test() {
        let mut precompiler = Precompiler::new();
        precompiler
            .functions
            .insert(Identifier {
                name: "name_1".to_string(),
            })
            .unwrap();

        let function_call = FunctionCall {
            name: Identifier {
                name: "name_1".to_string(),
            },
            args: vec![],
        };

        assert_eq!(
            function_call.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionCall {
                name: "name_1".to_string(),
            })
        );
    }

    #[test]
    fn precompile_function_call_error() {
        let mut precompiler = Precompiler::new();

        let function_call = FunctionCall {
            name: Identifier {
                name: "name_1".to_string(),
            },
            args: vec![],
        };

        assert_eq!(
            function_call.precompile(&mut precompiler),
            Err(precompiler::Error::UndefinedFunction(Identifier {
                name: "name_1".to_string(),
            }))
        );
    }
}
