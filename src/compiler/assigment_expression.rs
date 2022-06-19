use super::{Compile, Compiler, Error};
use crate::ast::{AssigmentExpression, Literal, RightAssigmentValue};
use inkwell::module::Module;

impl<'ctx> Compile<'ctx> for AssigmentExpression {
    fn compile(self, compiler: &mut Compiler<'ctx>, _: &Module<'ctx>) -> Result<(), Error> {
        match compiler.variables.get(&self.left).cloned() {
            Some(pointer) => match self.right {
                RightAssigmentValue::Literal(literal) => {
                    match literal {
                        Literal::Number(number) => {
                            let number = compiler.context.f64_type().const_float(number);
                            compiler.builder.build_store(pointer, number)
                        }
                        Literal::String(string) => {
                            let string = compiler.context.const_string(string.as_bytes(), false);
                            compiler.builder.build_store(pointer, string)
                        }
                    };
                    Ok(())
                }
                RightAssigmentValue::Identifier(identifier) => {
                    match compiler.variables.get(&identifier).cloned() {
                        Some(pointer) => {
                            compiler
                                .variables
                                .update(self.left.clone(), pointer)
                                .unwrap();
                            Ok(())
                        }
                        None => Err(Error::UndefinedVariable(identifier)),
                    }
                }
            },
            None => Err(Error::UndefinedVariable(self.left.clone())),
        }
    }
}
