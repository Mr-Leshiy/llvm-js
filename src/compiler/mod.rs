use crate::ast::{Expression, Identifier, Program};
use inkwell::{builder::Builder, context::Context, module::Module, values::PointerValue};
use std::collections::HashMap;
use thiserror::Error;

mod assigment_expression;
mod literal;
mod variable_declaration;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Undefined variable, {0}")]
    UndefinedVariable(Identifier),
}

pub trait CompileResult {
    fn to_string(&self) -> String;
}

pub trait Compile<'ctx> {
    type Output: CompileResult;
    // TODO: consider to the change to the 'self' instead of '&self'
    fn compile(&self, compiler: &mut Compiler<'ctx>) -> Result<Self::Output, Error>;
}

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,

    variables: HashMap<Identifier, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: String) -> Self {
        Self {
            context,
            builder: context.create_builder(),
            module: context.create_module(module_name.as_str()),

            variables: HashMap::new(),
        }
    }
}

impl CompileResult for () {
    fn to_string(&self) -> String {
        "".to_string()
    }
}

impl<'ctx> Compile<'ctx> for Program {
    type Output = ();

    fn compile(&self, compiler: &mut Compiler<'ctx>) -> Result<Self::Output, Error> {
        // create entry, point main function
        let func = compiler.module.add_function(
            "main",
            compiler.context.void_type().fn_type(&[], false),
            None,
        );
        let block = compiler.context.append_basic_block(func, "entry");
        compiler.builder.position_at_end(block);

        for expr in &self.body {
            match expr {
                Expression::AssigmentExpression(assigment_expression) => {
                    assigment_expression.compile(compiler)?
                }
                Expression::VariableDeclaration(variable_declaration) => {
                    variable_declaration.compile(compiler)?
                }
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Module;

    #[test]
    fn compile_program_from_file() {
        let file = std::fs::File::open("test_scripts/basic.js").unwrap();
        let module = Module::new("module_1".to_string(), file).unwrap();
        let context = Context::create();
        let mut compiler = Compiler::new(&context, module.name);

        module.program.compile(&mut compiler).unwrap();
    }
}
