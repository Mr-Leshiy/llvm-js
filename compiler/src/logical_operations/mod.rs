use crate::{Compiler, Error, Function, Variable};

pub fn logical_not<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _var: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement");
}

pub fn logical_and<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement");
}

pub fn logical_or<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement");
}
