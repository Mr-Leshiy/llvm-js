use crate::llvm_ast;
pub use binary_expression::{BinaryExpType, BinaryExpression};
pub use block_statement::BlockStatement;
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
use lexer::TokenReader;
use precompiler::{self, Precompiler};
pub use program::Program;
use std::io::Read;
pub use unary_expression::{UnaryExpType, UnaryExpression};
pub use variable_assigment::VariableAssigment;
pub use variable_declaration::VariableDeclaration;
pub use variable_expression::VariableExpression;
pub use variable_value::VariableValue;

mod binary_expression;
mod block_statement;
mod expression;
mod function_call;
mod function_declaration;
mod identifier;
mod program;
mod unary_expression;
mod variable_assigment;
mod variable_declaration;
mod variable_expression;
mod variable_value;

/// Module
pub struct Module {
    pub name: String,
    pub program: Program,
}

impl Module {
    pub fn new<R: Read>(name: String, input: R) -> Result<Self, lexer::Error> {
        let mut reader = TokenReader::new(input);
        let program = Program::parse(reader.next_token()?, &mut reader)?;
        Ok(Self { name, program })
    }

    pub fn precompile<Iter>(
        self,
        predefined_functions: Iter,
    ) -> Result<llvm_ast::Module, precompiler::Error<Identifier>>
    where
        Iter: Iterator<Item = Identifier>,
    {
        let precompiler = Precompiler::new(predefined_functions);

        Ok(llvm_ast::Module {
            name: self.name,
            program: self.program.precompile(precompiler)?,
        })
    }
}
