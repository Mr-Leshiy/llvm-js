use super::Identifier;
use crate::{
    lexer::{Separator, Token, TokenReader},
    parser::{self, Parser},
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
}
