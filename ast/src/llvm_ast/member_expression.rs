use super::{FunctionCall, Identifier, VariableExpression, VariableValue};
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
    fn compile_get_variable<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        variable: &Variable<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let variable = match self.object {
            PropertyType::Identifier(identifier) => {
                variable.get_property_by_str(compiler, String::from(identifier).as_str())
            }
            PropertyType::FunctionCall(function_call) => {
                let mut args = Vec::new();
                for arg in function_call.args {
                    let value = arg.compile_get_variable(compiler)?;
                    let arg = Variable::new_undefined(compiler, true);
                    arg.assign_variable(compiler, &value);
                    if value.is_tmp() {
                        value.deallocate(compiler);
                    }
                    args.push(arg);
                }

                let var = variable
                    .get_property_by_str(compiler, String::from(function_call.name).as_str());
                let ret = var.function_call(compiler, &args);

                // deallocate arguments
                for arg in args {
                    arg.deallocate(compiler);
                }
                ret
            }
            PropertyType::VariableExpression(VariableExpression::VariableValue(
                VariableValue::Boolean(key),
            )) => {
                let res = variable.get_property_by_boolean(compiler, key);
                res
            }
            PropertyType::VariableExpression(VariableExpression::VariableValue(
                VariableValue::FloatNumber(key),
            )) => {
                let res = variable.get_property_by_number(compiler, key);
                res
            }
            PropertyType::VariableExpression(VariableExpression::VariableValue(
                VariableValue::String(key),
            )) => {
                let res = variable.get_property_by_str(compiler, &key);
                res
            }
            PropertyType::VariableExpression(variable_expression) => {
                let key = variable_expression.compile_get_variable(compiler)?;
                let res = variable.get_property_by_var(compiler, &key);
                if key.is_tmp() {
                    key.deallocate(compiler);
                }
                res
            }
        };
        if let Some(property) = self.property {
            property.compile_get_variable(compiler, &variable)
        } else {
            Ok(variable)
        }
    }

    fn compile_update_variable<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        variable: &Variable<'ctx>,
        new_value: &Variable<'ctx>,
    ) -> Result<(), CompilerError> {
        if let Some(property) = self.property.clone() {
            let variable = self.compile_get_variable(compiler, variable)?;
            property.compile_update_variable(compiler, &variable, new_value)?;
        } else {
            match self.object {
                PropertyType::Identifier(identifier) => {
                    variable.add_property_by_str(
                        compiler,
                        String::from(identifier).as_str(),
                        new_value,
                        true,
                    );
                }
                PropertyType::FunctionCall(function_call) => {
                    let mut args = Vec::new();
                    for arg in function_call.args {
                        let value = arg.compile_get_variable(compiler)?;
                        let arg = Variable::new_undefined(compiler, true);
                        arg.assign_variable(compiler, &value);
                        if value.is_tmp() {
                            value.deallocate(compiler);
                        }
                        args.push(arg);
                    }

                    let var = variable
                        .get_property_by_str(compiler, String::from(function_call.name).as_str());
                    let _: Variable = var.function_call(compiler, &args);

                    // deallocate arguments
                    for arg in args {
                        arg.deallocate(compiler);
                    }
                }
                PropertyType::VariableExpression(VariableExpression::VariableValue(
                    VariableValue::Boolean(key),
                )) => {
                    variable.add_property_by_boolean(compiler, key, new_value, true);
                }
                PropertyType::VariableExpression(VariableExpression::VariableValue(
                    VariableValue::FloatNumber(key),
                )) => {
                    variable.add_property_by_number(compiler, key, new_value, true);
                }
                PropertyType::VariableExpression(VariableExpression::VariableValue(
                    VariableValue::String(key),
                )) => {
                    variable.add_property_by_str(compiler, &key, new_value, true);
                }
                PropertyType::VariableExpression(variable_expression) => {
                    let key = variable_expression.compile_get_variable(compiler)?;
                    variable.add_property_by_var(compiler, &key, new_value, true);
                    if key.is_tmp() {
                        key.deallocate(compiler);
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub object: Box<VariableExpression>,
    pub property: Property,
}

impl MemberExpression {
    pub fn compile_get_variable<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        let variable = self.object.compile_get_variable(compiler)?;
        self.property.compile_get_variable(compiler, &variable)
    }

    pub fn compile_update_variable<'ctx>(
        self,
        compiler: &mut Compiler<'ctx>,
        new_value: &Variable<'ctx>,
    ) -> Result<(), CompilerError> {
        let variable = self.object.compile_get_variable(compiler)?;
        self.property
            .compile_update_variable(compiler, &variable, new_value)?;
        Ok(())
    }
}
