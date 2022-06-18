pub use keyword::Keyword;
pub use literal::Literal;
pub use separator::Separator;
use std::fmt::Display;

mod keyword;
mod literal;
mod separator;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// keyword token
    Keyword(Keyword),
    /// assign token, "="
    Assign,
    /// ident token, e.g. "val1", "car_type"
    Ident(String),
    /// literal token
    Literal(Literal),
    /// separator token,
    Separator(Separator),
    /// end of file token
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keyword(val) => val.fmt(f),
            Self::Assign => write!(f, "Assign token"),
            Self::Ident(val) => write!(f, "Ident token, val: {}", val),
            Self::Literal(val) => val.fmt(f),
            Self::Separator(val) => val.fmt(f),
            Self::Eof => write!(f, "Eof token"),
        }
    }
}
