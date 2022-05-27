use super::{
    identifier::parse_identifier, right_assignment_value::parse_right_assignment_value, Error,
};
use crate::{
    ast::AssigmentExpression,
    lexer::{CharReader, Token},
};
use std::io::Read;

pub fn parse_assignment_expression<R: Read>(
    reader: &mut CharReader<R>,
) -> Result<AssigmentExpression, Error> {
    let left = parse_identifier(reader)?;
    match Token::get_token(reader)? {
        Token::Assign => {}
        token => return Err(Error::UnexpectedToken(token)),
    }

    let right = parse_right_assignment_value(reader)?;
    Ok(AssigmentExpression { left, right })
}
