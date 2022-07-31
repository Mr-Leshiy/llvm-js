use crate::{Compiler, Error, Function, Variable};

pub fn logical_not<'ctx>(
    _compiler: &Compiler<'ctx>,
    _cur_function: &Function<'ctx>,
    var: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error> {
    // TODO implement
    Ok(var)
}

pub fn logical_and<'ctx>(
    _compiler: &Compiler<'ctx>,
    _cur_function: &Function<'ctx>,
    var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error> {
    // TODO implement
    Ok(var1)
}

pub fn logical_or<'ctx>(
    _compiler: &Compiler<'ctx>,
    _cur_function: &Function<'ctx>,
    var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error> {
    // TODO implement
    Ok(var1)
}
