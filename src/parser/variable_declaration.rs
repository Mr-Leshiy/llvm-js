use super::{
    identifier::parse_identifier, right_assignment_value::parse_right_assignment_value, Error,
};
use crate::{
    ast::VariableDeclaration,
    lexer::{CharReader, Token},
};
use std::io::Read;

pub fn parse_variable_declaration<R: Read>(
    reader: &mut CharReader<R>,
) -> Result<VariableDeclaration, Error> {
    match Token::get_token(reader)? {
        Token::Var => {
            let id = parse_identifier(reader)?;
            match Token::get_token(reader)? {
                Token::Assign => {}
                token => return Err(Error::UnexpectedToken(token)),
            }
            let init = parse_right_assignment_value(reader)?;
            Ok(VariableDeclaration { id, init })
        }
        token => Err(Error::UnexpectedToken(token)),
    }
}
