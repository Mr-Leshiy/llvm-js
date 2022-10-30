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
        cur_function: &mut Function<'ctx, Identifier>,
        variable: &Variable<'ctx>,
        allocate: bool,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let key = self.object.compile(compiler, cur_function)?;
        let variable = variable.get_property_by_var(compiler, &key, allocate)?;
        if let Some(property) = self.property {
            property.compile(compiler, cur_function, &variable, allocate)
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
        allocate: bool,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        let variable = cur_function.get_variable(self.variable_name)?;
        if let Some(property) = self.property {
            property.compile(compiler, cur_function, &variable, allocate)
        } else {
            Ok(variable)
        }
    }
}
