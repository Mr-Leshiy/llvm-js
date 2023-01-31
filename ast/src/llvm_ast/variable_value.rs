use super::{ArrayExpression, MemberExpression, ObjectExpression};
use crate::{Compiler, CompilerError};
use compiler::Variable;

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
        compiler: &mut Compiler<'ctx>,
    ) -> Result<Variable<'ctx>, CompilerError> {
        match self {
            Self::Undefined => Ok(Variable::new_undefined(compiler, true)),
            Self::Null => Ok(Variable::new_null(compiler, true)),
            Self::NaN => Ok(Variable::new_nan(compiler, true)),
            Self::Infinity => Ok(Variable::new_infinity(compiler, true)),
            Self::NegInfinity => Ok(Variable::new_neginfinity(compiler, true)),
            Self::Boolean(boolean) => Ok(Variable::new_boolean(compiler, boolean, true)),
            Self::String(string) => Ok(Variable::new_string(compiler, &string, true)),
            Self::FloatNumber(number) => Ok(Variable::new_number(compiler, number, true)),
            Self::MemberExpression(member_expression) => member_expression.compile(compiler, false),
            Self::ObjectExpression(object_expression) => object_expression.compile(compiler),
            Self::ArrayExpression(array_expression) => array_expression.compile(compiler),
        }
    }
}
