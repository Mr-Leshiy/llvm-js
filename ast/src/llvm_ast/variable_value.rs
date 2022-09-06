use super::Identifier;
use compiler::{self, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Boolean(bool),
    FloatNumber(f64),
    String(String),
    Identifier(Identifier),
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
            VariableValue::Identifier(name) => Ok(Variable::new_variable(
                compiler,
                cur_function,
                &cur_function.get_variable(name)?,
            )),
        }
    }
}
