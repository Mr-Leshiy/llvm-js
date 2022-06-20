use super::{BlockStatement, Identifier};
use crate::{
    compiler::{self, Compile, Compiler},
    lexer::{self, CharReader, Keyword, Separator, Token},
    parser::{self, Parser},
};
use inkwell::module::Module;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Parser for FunctionDeclaration {
    fn parse<R: Read>(
        mut cur_token: Token,
        reader: &mut CharReader<R>,
    ) -> Result<Self, parser::Error> {
        match cur_token {
            Token::Keyword(Keyword::Function) => {
                // parse function name
                let name = Identifier::parse(lexer::get_token(reader)?, reader)?;

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

                // parse function body
                let body = BlockStatement::parse(lexer::get_token(reader)?, reader)?;

                Ok(Self { name, args, body })
            }
            token => Err(parser::Error::UnexpectedToken(token)),
        }
    }
}

impl<'ctx> Compile<'ctx> for FunctionDeclaration {
    fn compile(self, _: &mut Compiler<'ctx>, _: &Module<'ctx>) -> Result<(), compiler::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_declaration_test() {
        let mut reader = CharReader::new("function foo() {}".as_bytes());
        assert_eq!(
            FunctionDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader)
                .unwrap(),
            FunctionDeclaration {
                name: Identifier {
                    name: "foo".to_string()
                },
                args: vec![],
                body: BlockStatement { body: vec![] }
            }
        );
    }
}
