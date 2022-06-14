use super::{Compile, Compiler, Error};
use crate::ast::Expression;
use inkwell::module::Module;

impl<'ctx> Compile<'ctx> for Expression {
    type Output = ();

    fn compile(
        &self,
        compiler: &mut Compiler<'ctx>,
        module: &Module<'ctx>,
    ) -> Result<Self::Output, Error> {
        match self {
            Expression::Assigment(assigment_expression) => {
                assigment_expression.compile(compiler, module)?;
            }
            Expression::VariableDeclaration(variable_declaration) => {
                let pointer = compiler.builder.build_alloca(
                    compiler.context.f64_type(),
                    variable_declaration.left.name.as_str(),
                );
                compiler
                    .variables
                    .insert(variable_declaration.left.clone(), pointer);

                variable_declaration.compile(compiler, module)?;
            }
            Expression::BlockStatement { body } => {
                for expr in body {
                    expr.compile(compiler, module)?;
                }
            }
        };
        Ok(())
    }
}
