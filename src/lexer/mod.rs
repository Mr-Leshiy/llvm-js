use self::char_reader::CharReader;
use thiserror::Error;

mod char_reader;

#[derive(Debug, Error)]
enum Error {}

enum Token {
    /// "var"
    Var,
    /// assign token, "="
    Assign,
    /// ident token, e.g. "val1", "car_type"
    Ident(String),
    /// number token, e.g. 5, 6, 6.12
    Number(f64),
    /// end of file token
    Eof,
}

impl Token {
    fn get_next_token<R: std::io::Read>(reader: &mut CharReader<R>) -> Result<Self, Error> {
        // Skip whitespaces

        Ok(Self::Eof)
    }
}

#[cfg(test)]
mod tests {}
