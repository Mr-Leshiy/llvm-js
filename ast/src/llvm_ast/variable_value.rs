use super::Identifier;
use compiler::{self, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Undefined,
    Null,
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
            Self::Undefined => Variable::new_undefined(compiler),
            Self::Null => Variable::new_null(compiler),
            Self::Boolean(boolean) => Variable::new_boolean(compiler, boolean),
            Self::String(string) => Variable::new_string(compiler, &string),
            Self::FloatNumber(number) => Variable::new_number(compiler, number),
            Self::Identifier(name) => {
                Variable::new_variable(compiler, &cur_function.get_variable(name)?)
            }
        }
    }
}
