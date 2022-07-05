use super::{BlockStatement, Identifier};
use crate::{
    lexer::{self, CharReader, Keyword, Separator, Token},
    llvm_ast,
    parser::{self, Parser},
    precompiler::{self, Precompile, Precompiler},
};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
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

impl Precompile for FunctionDeclaration {
    type Output = llvm_ast::FunctionDeclaration;
    fn precompile(self, precompiler: &mut Precompiler) -> Result<Self::Output, precompiler::Error> {
        precompiler
            .functions
            .insert(self.name.clone())
            .map_err(|_| precompiler::Error::AlreadyDeclaredFunction(self.name.clone()))?;
        Ok(llvm_ast::FunctionDeclaration {
            name: self.name.name,
            body: self.body.precompile(precompiler)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_function_declaration_test() {
        let mut reader = CharReader::new("function foo(a, b) {}".as_bytes());
        assert_eq!(
            FunctionDeclaration::parse(lexer::get_token(&mut reader).unwrap(), &mut reader),
            Ok(FunctionDeclaration {
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
                ],
                body: BlockStatement { body: vec![] }
            })
        );
    }

    #[test]
    fn precompile_function_declaration_test() {
        let mut precompiler = Precompiler::new();

        let function_declaration = FunctionDeclaration {
            name: Identifier {
                name: "name_1".to_string(),
            },
            args: vec![],
            body: BlockStatement { body: vec![] },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Ok(llvm_ast::FunctionDeclaration {
                name: "name_1".to_string(),
                body: vec![]
            })
        );
        assert!(precompiler.functions.contains(&Identifier {
            name: "name_1".to_string(),
        }));
    }

    #[test]
    fn precompile_function_declaration_error_test() {
        let mut precompiler = Precompiler::new();
        precompiler
            .functions
            .insert(Identifier {
                name: "name_1".to_string(),
            })
            .unwrap();

        let function_declaration = FunctionDeclaration {
            name: Identifier {
                name: "name_1".to_string(),
            },
            args: vec![],
            body: BlockStatement { body: vec![] },
        };

        assert_eq!(
            function_declaration.precompile(&mut precompiler),
            Err(precompiler::Error::AlreadyDeclaredFunction(Identifier {
                name: "name_1".to_string(),
            }))
        );
    }
}
