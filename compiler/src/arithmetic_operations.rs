use crate::{
    Compiler, Error, Function, Variable,
};

pub fn arithmetic_add<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement");
}

pub fn arithmetic_sub<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement");
}

pub fn arithmetic_div<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement");
}

pub fn arithmetic_mul<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _var1: Variable<'ctx>,
    _var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement");
}
