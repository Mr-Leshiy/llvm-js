use super::{FunctionCall, Identifier, VariableExpression};
use crate::{Compiler, CompilerError};
use compiler::Variable;

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyType {
    Identifier(Identifier),
    FunctionCall(FunctionCall),
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
        variable: &Variable<'ctx>,
        allocate: bool,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let variable = match self.object {
            PropertyType::Identifier(identifier) => {
                variable.get_property_by_str(compiler, String::from(identifier).as_str(), allocate)
            }
            PropertyType::FunctionCall(function_call) => {
                let mut args = Vec::new();
                for arg in function_call.args {
                    let value = arg.compile(compiler)?;
                    let arg = Variable::new_undefined(compiler, true);
                    arg.assign_variable(compiler, &value);
                    if value.is_tmp() {
                        value.deallocate(compiler);
                    }
                    args.push(arg);
                }

                let var = variable.get_property_by_str(
                    compiler,
                    String::from(function_call.name).as_str(),
                    allocate,
                );
                let ret = var.function_call(compiler, &args);

                // deallocate arguments
                for arg in args {
                    arg.deallocate(compiler);
                }
                ret
            }
            PropertyType::VariableExpression(variable_expression) => {
                let key = variable_expression.compile(compiler)?;
                let res = variable.get_property_by_var(compiler, &key, allocate);
                if key.is_tmp() {
                    key.deallocate(compiler);
                }
                res
            }
        };
        if let Some(property) = self.property {
            property.compile(compiler, &variable, allocate)
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
        allocate: bool,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let variable = compiler.get_variable(self.variable_name)?;
        if let Some(property) = self.property {
            property.compile(compiler, &variable, allocate)
        } else {
            Ok(variable)
        }
    }
}
