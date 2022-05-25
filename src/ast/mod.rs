trait Expr {}

/// Literal - Expression type for numeric literals like "1.0" or string literals "hello".
enum Literal {
    Number(f64),
    String(String),
}

/// Identifier - Expression type for any identifier, like "name"
struct Identifier {
    name: String,
}

/// VariableDeclaration - Expression type for variable declaration, like "var a = 4"
struct VariableDeclaration {
    id: Identifier,
    init: Literal,
}
