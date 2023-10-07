use std::fmt::Display;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Identifier {
    name: String,
    index: u32,
}

impl Identifier {
    pub fn new(name: String, index: u32) -> Self {
        Self { name, index }
    }
}

impl From<Identifier> for String {
    fn from(val: Identifier) -> Self {
        if val.index == 0 {
            val.name
        } else {
            format!("{}{}", val.name, val.index)
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, index: {}", self.name, self.index)
    }
}
