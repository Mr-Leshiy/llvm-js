use super::{Identifier, LogicalExpression};
use compiler::{self, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Boolean(bool),
    FloatNumber(f64),
    String(String),
    Identifier(Identifier),
    LogicalExpression(Box<LogicalExpression>),
}

impl VariableValue {
    pub fn compile<'ctx>(
        self,
        compiler: &mut Compiler<'ctx, Identifier>,
        cur_function: &mut Function<'ctx, Identifier>,
    ) -> Result<Variable<'ctx>, compiler::Error<Identifier>> {
        match self {
            VariableValue::Boolean(boolean) => Ok(Variable::new_boolean(compiler, boolean)),
            VariableValue::String(string) => Ok(Variable::new_string(compiler, &string)),
            VariableValue::FloatNumber(number) => Ok(Variable::new_number(compiler, number)),
            VariableValue::Identifier(name) => cur_function.get_variable(name),
            VariableValue::LogicalExpression(logical) => logical.compile(compiler, cur_function),
        }
    }
}
