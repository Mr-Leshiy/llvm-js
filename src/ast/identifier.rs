use std::fmt::Display;

/// Identifier - Expression type for any identifier, like "name"
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier {
    pub name: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}", self.name)
    }
}
