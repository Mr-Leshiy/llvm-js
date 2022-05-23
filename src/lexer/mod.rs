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
    EOF,
}

#[cfg(test)]
mod tests {}
