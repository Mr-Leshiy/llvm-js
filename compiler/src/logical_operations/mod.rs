use crate::{Compiler, Error, Function, Variable, VariableValue};

pub fn logical_not<'ctx>(
    compiler: &Compiler<'ctx>,
    cur_function: &Function<'ctx>,
    var: VariableValue,
) -> Result<Variable<'ctx>, Error> {
    // TODO implement
    let var = Variable::try_from_variable_value(compiler, cur_function, var)?;
    Ok(var)
}

pub fn logical_and<'ctx>(
    compiler: &Compiler<'ctx>,
    cur_function: &Function<'ctx>,
    var1: VariableValue,
    _var2: VariableValue,
) -> Result<Variable<'ctx>, Error> {
    // TODO implement
    let var = Variable::try_from_variable_value(compiler, cur_function, var1)?;
    Ok(var)
}

pub fn logical_or<'ctx>(
    compiler: &Compiler<'ctx>,
    cur_function: &Function<'ctx>,
    var1: VariableValue,
    _var2: VariableValue,
) -> Result<Variable<'ctx>, Error> {
    // TODO implement
    let var = Variable::try_from_variable_value(compiler, cur_function, var1)?;
    Ok(var)
}
