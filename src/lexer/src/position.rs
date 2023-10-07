use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    line: u64,
    column: u64,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(line: {0}, column: {1})", self.line, self.column)
    }
}

impl Position {
    pub fn new(line: u64, column: u64) -> Self {
        Self { line, column }
    }

    pub fn inc_line(&mut self) {
        self.line += 1;
    }

    pub fn inc_column(&mut self) {
        self.line = 1;
        self.column += 1;
    }
}
