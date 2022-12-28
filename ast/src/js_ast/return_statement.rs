use super::VariableExpression;
use crate::{llvm_ast, Error, Precompiler};
use lexer::{Keyword, Token, TokenReader};
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    ret: VariableExpression,
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
        precompiler: &mut Precompiler,
    ) -> Result<llvm_ast::ReturnStatement, Error> {
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
