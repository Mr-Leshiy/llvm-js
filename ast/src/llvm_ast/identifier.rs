#[derive(Debug, Clone, PartialEq)]
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
