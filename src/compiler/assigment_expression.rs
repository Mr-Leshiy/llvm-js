use super::{literal::CompiledLiteral, Compile, CompileResult, Compiler, Error};
use crate::ast::{AssigmentExpression, RightAssigmentValue};
use inkwell::{
    module::Module,
    values::{AnyValue, PointerValue},
};

impl<'ctx> CompileResult for PointerValue<'ctx> {
    fn to_string(&self) -> String {
        self.print_to_string().to_string()
    }
}

impl<'ctx> Compile<'ctx> for AssigmentExpression {
    type Output = PointerValue<'ctx>;

    fn compile(
        self,
        compiler: &mut Compiler<'ctx>,
        module: &Module<'ctx>,
    ) -> Result<Self::Output, Error> {
        match compiler.variables.get(&self.left).cloned() {
            Some(pointer) => match self.right {
                RightAssigmentValue::Literal(literal) => {
                    match literal.compile(compiler, module)? {
                        CompiledLiteral::Number(number) => {
                            compiler.builder.build_store(pointer, number)
                        }
                        CompiledLiteral::String(string) => {
                            compiler.builder.build_store(pointer, string)
                        }
                    };

                    Ok(pointer)
                }
                RightAssigmentValue::Identifier(identifier) => {
                    match compiler.variables.get(&identifier).cloned() {
                        Some(pointer) => {
                            compiler
                                .variables
                                .update(self.left.clone(), pointer)
                                .unwrap();
                            Ok(pointer)
                        }
                        None => Err(Error::UndefinedVariable(identifier)),
                    }
                }
            },
            None => Err(Error::UndefinedVariable(self.left.clone())),
        }
    }
}
