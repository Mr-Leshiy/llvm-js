use crate::{Compiler, Error, Function, Variable};

pub fn logical_not<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    var: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    // TODO implement
    Ok(var)
}

pub fn logical_and<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    // TODO implement
    Ok(var1)
}

pub fn logical_or<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    // TODO implement
    Ok(var1)
}
