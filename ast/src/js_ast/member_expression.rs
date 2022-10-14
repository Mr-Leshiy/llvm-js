use super::Identifier;
use crate::{llvm_ast, Error};
use lexer::{Separator, Token, TokenReader};
use precompiler::Precompiler;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct MemberExpression {
    pub object: Identifier,
    pub property: Option<Box<MemberExpression>>,
}

impl MemberExpression {
    fn parse_impl<R: Read>(
        cur_token: Token,
        reader: &mut TokenReader<R>,
    ) -> Result<Option<Box<Self>>, Error> {
        match cur_token {
            Token::Separator(Separator::Dot) => {
                let object = Identifier::parse(reader.next_token()?, reader)?;
                reader.start_saving();
                let property = Self::parse_impl(reader.next_token()?, reader)?;
                Ok(Some(Self { object, property }.into()))
            }
            _ => {
                reader.stop_saving();
                Ok(None)
            }
        }
    }

    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        let object = Identifier::parse(cur_token, reader)?;
        reader.start_saving();
        let property = Self::parse_impl(reader.next_token()?, reader)?;
        Ok(Self { object, property })
    }
}

impl MemberExpression {
    fn precompile_impl(self) -> llvm_ast::MemberExpression {
        llvm_ast::MemberExpression {
            object: llvm_ast::Identifier::new(self.object.name, 0),
            property: self
                .property
                .map(|property| property.precompile_impl().into()),
        }
    }

    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::MemberExpression, precompiler::Error<Identifier>> {
        match precompiler.variables.get(&self.object) {
            Some(index) => Ok(llvm_ast::MemberExpression {
                object: llvm_ast::Identifier::new(self.object.name, index),
                property: self
                    .property
                    .map(|property| property.precompile_impl().into()),
            }),
            None => Err(precompiler::Error::UndefinedVariable(self.object.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_member_expression_test() {
        let mut reader = TokenReader::new("name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                object: "name".to_string().into(),
                property: None
            }),
        );

        let mut reader = TokenReader::new("name.name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                object: "name".to_string().into(),
                property: Some(
                    MemberExpression {
                        object: "name".to_string().into(),
                        property: None
                    }
                    .into()
                )
            }),
        );

        let mut reader = TokenReader::new("name.name.name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                object: "name".to_string().into(),
                property: Some(
                    MemberExpression {
                        object: "name".to_string().into(),
                        property: Some(
                            MemberExpression {
                                object: "name".to_string().into(),
                                property: None
                            }
                            .into()
                        )
                    }
                    .into()
                )
            }),
        );

        let mut reader = TokenReader::new("name.name.name.name".as_bytes());
        assert_eq!(
            MemberExpression::parse(reader.next_token().unwrap(), &mut reader),
            Ok(MemberExpression {
                object: "name".to_string().into(),
                property: Some(
                    MemberExpression {
                        object: "name".to_string().into(),
                        property: Some(
                            MemberExpression {
                                object: "name".to_string().into(),
                                property: Some(
                                    MemberExpression {
                                        object: "name".to_string().into(),
                                        property: None
                                    }
                                    .into()
                                )
                            }
                            .into()
                        )
                    }
                    .into()
                )
            }),
        );
    }
}
