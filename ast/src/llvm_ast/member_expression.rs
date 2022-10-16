use super::{Identifier, VariableExpression};
use compiler::{Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub object: VariableExpression,
    pub property: Option<Box<Property>>,
}

impl Property {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        variable: &Variable<'ctx>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let get_property_fn = compiler.predefined_functions()?.get_property();
        // TODO fix compile
        let variable = get_property_fn.call(compiler, variable, "hello");
        if let Some(property) = self.property {
            property.compile(compiler, &variable)
        } else {
            Ok(variable)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub variable_name: Identifier,
    pub property: Option<Box<Property>>,
}

impl MemberExpression {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let variable =
            Variable::new_variable(compiler, &cur_function.get_variable(self.variable_name)?)?;
        if let Some(property) = self.property {
            property.compile(compiler, &variable)
        } else {
            Ok(variable)
        }
    }
}
