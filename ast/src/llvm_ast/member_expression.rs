use super::{Identifier, VariableExpression};
use crate::{Compiler, CompilerError, Function};
use compiler::Variable;

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyType {
    Identifier(Identifier),
    VariableExpression(VariableExpression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub object: PropertyType,
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
        let variable = match self.object {
            PropertyType::Identifier(identifier) => {
                variable.get_property_by_str(compiler, String::from(identifier).as_str(), allocate)
            }
            PropertyType::VariableExpression(variable_expression) => {
                let key = variable_expression.compile(compiler, cur_function)?;
                let res = variable.get_property_by_var(compiler, &key, allocate);
                if key.is_tmp() {
                    key.deallocate(compiler)?;
                }
                res
            }
        }?;
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
