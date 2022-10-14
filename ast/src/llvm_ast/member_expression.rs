use super::Identifier;
use compiler::{Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub object: Identifier,
    pub property: Option<Box<MemberExpression>>,
}

impl MemberExpression {
    fn compile_impl<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        variable: &Variable<'ctx>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let get_property_fn = compiler.predefined_functions()?.get_property();
        let variable = get_property_fn.call(compiler, variable, &String::from(self.object));
        if let Some(property) = self.property {
            property.compile_impl(compiler, &variable)
        } else {
            Ok(variable)
        }
    }

    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let variable = Variable::new_variable(compiler, &cur_function.get_variable(self.object)?)?;
        if let Some(property) = self.property {
            property.compile_impl(compiler, &variable)
        } else {
            Ok(variable)
        }
    }
}
