use crate::llvm_ast;
pub use block_statement::BlockStatement;
pub use expression::Expression;
pub use function_call::FunctionCall;
pub use function_declaration::FunctionDeclaration;
pub use identifier::Identifier;
use lexer::{Parser, TokenReader};
pub use logical_expression::LogicalExpression;
use precompiler::{self, Precompile, Precompiler};
pub use program::Program;
use std::io::Read;
pub use variable_assigment::VariableAssigment;
pub use variable_declaration::VariableDeclaration;
pub use variable_expression::VariableExpression;
pub use variable_value::VariableValue;

mod block_statement;
mod expression;
mod function_call;
mod function_declaration;
mod identifier;
mod logical_expression;
mod program;
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
        let mut precompiler = Precompiler::new(predefined_functions);

        let mut body = Vec::new();
        for expr in self.program.body {
            expr.precompile(&mut precompiler)?
                .into_iter()
                .for_each(|expr| body.push(expr));
        }

        Ok(llvm_ast::Module {
            name: self.name,
            program: llvm_ast::Program {
                functions: precompiler.function_declarations,
                body,
            },
        })
    }
}
