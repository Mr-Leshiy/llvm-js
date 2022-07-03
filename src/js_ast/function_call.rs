use super::Identifier;
use crate::{
    lexer::{self, CharReader, Separator, Token},
    parser::{self, Parser},
};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
struct FunctionCall {
    pub name: Identifier,
    pub args: Vec<Identifier>,
}

impl Parser for FunctionCall {
    fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut CharReader<R>,
    ) -> Result<Self, parser::Error> {
        // parse function name
        let name = Identifier::parse(cur_token, reader)?;

        // parse function args
        let args = match lexer::get_token(reader)? {
            Token::Separator(Separator::OpenBrace) => {
                let mut args = Vec::new();
                cur_token = lexer::get_token(reader)?;
                loop {
                    let arg = match cur_token {
                        Token::Separator(Separator::CloseBrace) => break,
                        cur_token => Identifier::parse(cur_token, reader)?,
                    };
                    args.push(arg);

                    cur_token = match lexer::get_token(reader)? {
                        Token::Separator(Separator::CloseBrace) => break,
                        Token::Separator(Separator::Comma) => lexer::get_token(reader)?,
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
        let mut reader = CharReader::new("foo(a, b)".as_bytes());
        assert_eq!(
            FunctionCall::parse(lexer::get_token(&mut reader).unwrap(), &mut reader).unwrap(),
            FunctionCall {
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
            }
        );
    }
}
