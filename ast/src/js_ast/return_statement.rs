use super::{Identifier, VariableExpression};
use crate::{llvm_ast, Error};
use lexer::{Keyword, Token, TokenReader};
use precompiler::Precompiler;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub ret: VariableExpression,
}

impl ReturnStatement {
    pub fn parse<R: Read>(cur_token: Token, reader: &mut TokenReader<R>) -> Result<Self, Error> {
        match cur_token {
            Token::Keyword(Keyword::Return) => Ok(Self {
                ret: VariableExpression::parse(reader.next_token()?, reader)?,
            }),
            cur_token => Err(Error::UnexpectedToken(cur_token)),
        }
    }
}

impl ReturnStatement {
    pub fn precompile(
        self,
        precompiler: &mut Precompiler<Identifier, llvm_ast::FunctionDeclaration>,
    ) -> Result<llvm_ast::ReturnStatement, precompiler::Error<Identifier>> {
        Ok(llvm_ast::ReturnStatement {
            ret: self.ret.precompile(precompiler)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::js_ast::{VariableExpression, VariableValue};

    #[test]
    fn parse_return_statement_test() {
        let mut reader = TokenReader::new("return null;".as_bytes());
        assert_eq!(
            ReturnStatement::parse(reader.next_token().unwrap(), &mut reader),
            Ok(ReturnStatement {
                ret: VariableExpression::VariableValue(VariableValue::Null)
            })
        );
    }
}
