use super::Identifier;
use crate::{
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use lexer::{Separator, Token, TokenReader};
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
            // check if arguments exist
            for arg in &self.args {
                if !precompiler.variables.contains(arg) {
                    return Err(precompiler::Error::UndefinedVariable(arg.clone()));
                }
            }

            Ok(llvm_ast::FunctionCall {
                name: self.name.name,
                args: self.args.into_iter().map(|name| name.name).collect(),
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
                name: "foo".to_string().into(),
                args: vec!["a".to_string().into(), "b".to_string().into()]
            })
        );
    }

    #[test]
    fn precompile_function_call_test() {
        let mut precompiler = Precompiler::new(Vec::new().into_iter());
        precompiler
            .functions
            .insert("name_1".to_string().into())
            .unwrap();
        precompiler
            .variables
            .insert("a".to_string().into())
            .unwrap();
        precompiler
            .variables
            .insert("b".to_string().into())
            .unwrap();

        let function_call = FunctionCall {
            name: "name_1".to_string().into(),

            args: vec!["a".to_string().into(), "b".to_string().into()],
        };

        assert_eq!(
            function_call.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionCall {
                name: "name_1".to_string(),
                args: vec!["a".to_string(), "b".to_string()],
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
