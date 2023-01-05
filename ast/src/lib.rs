use lexer::Token;
use thiserror::Error;

pub mod js_ast;
pub mod llvm_ast;

type LexerError = lexer::Error;

type Precompiler = precompiler::Precompiler<js_ast::Identifier, llvm_ast::FunctionDeclaration>;
type PrecompilerError = precompiler::Error<js_ast::Identifier>;

type CompilerError = compiler::Error<llvm_ast::Identifier>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Unexpected token provided: {0}")]
    UnexpectedToken(Token),
    #[error(transparent)]
    LexerError(#[from] lexer::Error),
}
