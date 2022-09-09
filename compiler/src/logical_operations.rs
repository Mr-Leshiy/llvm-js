use crate::{Compiler, Error, Function, Variable};

pub fn logical_not<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _arg: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement")
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

pub fn logical_eq<'ctx, T>(
    compiler: &Compiler<'ctx, T>,
    cur_function: &Function<'ctx, T>,
    var1: Variable<'ctx>,
    var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    // TODO: regular eq should follow this rules https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Equality
    logical_seq(compiler, cur_function, var1, var2)
}

pub fn logical_ne<'ctx, T>(
    compiler: &Compiler<'ctx, T>,
    cur_function: &Function<'ctx, T>,
    var1: Variable<'ctx>,
    var2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    // TODO: regular eq should follow this rules https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Inequality
    logical_sne(compiler, cur_function, var1, var2)
}

pub fn logical_seq<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _arg1: Variable<'ctx>,
    _arg2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement")
}

pub fn logical_sne<'ctx, T>(
    _compiler: &Compiler<'ctx, T>,
    _cur_function: &Function<'ctx, T>,
    _arg1: Variable<'ctx>,
    _arg2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    todo!("implement")
}
