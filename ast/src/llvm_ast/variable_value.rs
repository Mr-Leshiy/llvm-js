use super::{ArrayExpression, Identifier, MemberExpression, ObjectExpression};
use compiler::{self, Compiler, Function, Variable};

#[derive(Debug, Clone, PartialEq)]
pub enum VariableValue {
    Undefined,
    Null,
    NaN,
    Infinity,
    NegInfinity,
    Boolean(bool),
    FloatNumber(f64),
    String(String),
    MemberExpression(MemberExpression),
    ObjectExpression(ObjectExpression),
    ArrayExpression(ArrayExpression),
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
            Self::NaN => Variable::new_nan(compiler),
            Self::Infinity => Variable::new_infinity(compiler),
            Self::NegInfinity => Variable::new_neginfinity(compiler),
            Self::Boolean(boolean) => Variable::new_boolean(compiler, boolean),
            Self::String(string) => Variable::new_string(compiler, &string),
            Self::FloatNumber(number) => Variable::new_number(compiler, number),
            Self::MemberExpression(member_expression) => {
                member_expression.compile(compiler, cur_function, false)
            }
            Self::ObjectExpression(object_expression) => {
                object_expression.compile(compiler, cur_function)
            }
            Self::ArrayExpression(array_expression) => {
                array_expression.compile(compiler, cur_function)
            }
        }
    }
}
