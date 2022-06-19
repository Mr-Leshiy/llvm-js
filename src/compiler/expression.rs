use super::{Compile, Compiler, Error};
use crate::ast::{BlockStatement, Expression};
use inkwell::module::Module;

impl<'ctx> Compile<'ctx> for Expression {
    fn compile(self, compiler: &mut Compiler<'ctx>, module: &Module<'ctx>) -> Result<(), Error> {
        match self {
            Expression::FunctionDeclaration(function_declaration) => {}
            Expression::VariableDeclaration(variable_declaration) => {
                variable_declaration.compile(compiler, module)?;
            }
            Expression::Assigment(assigment_expression) => {
                assigment_expression.compile(compiler, module)?;
            }
            Expression::BlockStatement(BlockStatement { body }) => {
                // TODO: update LLVM IR compilation, need to handle variables allocation/dealocation for the BlockStatement case
                let variables_count = compiler.variables.len();
                for expr in body {
                    expr.compile(compiler, module)?;
                }
                compiler.variables.remove_last_added(variables_count);
            }
        };
        Ok(())
    }
}
