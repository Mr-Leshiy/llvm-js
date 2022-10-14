use super::Identifier;
use crate::{llvm_ast, Error};
use lexer::{Token, TokenReader};
use precompiler::Precompiler;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct MemberExpression {
    pub object: Identifier,
    pub property: Option<Box<MemberExpression>>,
}

impl MemberExpression {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        let object = Identifier::parse(cur_token, reader)?;
        Ok(Self {
            object,
            property: None,
        })
    }
}

impl MemberExpression {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::MemberExpression, precompiler::Error<Identifier>> {
        match precompiler.variables.get(&self.object) {
            Some(index) => Ok(llvm_ast::MemberExpression {
                object: llvm_ast::Identifier::new(self.object.name, index),
                property: None,
            }),
            None => Err(precompiler::Error::UndefinedVariable(self.object.clone())),
        }
    }
}
