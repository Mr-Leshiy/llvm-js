use super::{Identifier, VariableExpression};
use crate::{Compiler, CompilerError, Function};
use compiler::Variable;

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub object: VariableExpression,
    pub property: Option<Box<Property>>,
}

impl Property {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
        variable: &Variable<'ctx>,
        allocate: bool,
    ) -> Result<Variable<'ctx>, CompilerError> {
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
        compiler: &mut Compiler<'ctx>,
        cur_function: &mut Function<'ctx>,
        allocate: bool,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let variable = cur_function.get_variable(self.variable_name)?;
        if let Some(property) = self.property {
            property.compile(compiler, cur_function, &variable, allocate)
        } else {
            Ok(variable)
        }
    }
}
