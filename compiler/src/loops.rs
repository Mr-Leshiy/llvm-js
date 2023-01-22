use crate::{Compile, Compiler, Error, Variable};

pub fn generate_while_loop<'ctx, T, Expr: Compile<T, Output = bool>>(
    compiler: &mut Compiler<'ctx, T>,
    condition: impl FnOnce(&mut Compiler<'ctx, T>) -> Result<Variable<'ctx>, Error<T>>,
    body: Vec<Expr>,
) -> Result<(), Error<T>> {
    let condition_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");
    let body_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");
    let continue_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");

    compiler.builder.build_unconditional_branch(condition_block);

    // describe condition
    compiler.builder.position_at_end(condition_block);
    let condition = condition(compiler)?;
    let get_boolean_fn = compiler.predefined_functions()?.get_boolean();
    let condition = get_boolean_fn.call(compiler, &condition);
    let condition = compiler.builder.build_int_compare(
        inkwell::IntPredicate::EQ,
        condition,
        compiler.context.i8_type().const_int(1_u64, false),
        "",
    );
    compiler
        .builder
        .build_conditional_branch(condition, body_block, continue_block);

    // describe body
    compiler.builder.position_at_end(body_block);
    let mut is_returned = false;
    for expr in body {
        let is_return = expr.compile(compiler)?;
        if is_return {
            is_returned = true;
            break;
        }
    }
    if !is_returned {
        compiler.builder.build_unconditional_branch(condition_block);
    }

    //
    compiler.builder.position_at_end(continue_block);

    Ok(())
}

pub fn generate_do_while_loop<'ctx, T, Expr: Compile<T, Output = bool>>(
    compiler: &mut Compiler<'ctx, T>,
    condition: impl FnOnce(&mut Compiler<'ctx, T>) -> Result<Variable<'ctx>, Error<T>>,
    body: Vec<Expr>,
) -> Result<(), Error<T>> {
    let condition_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");
    let body_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");
    let continue_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");

    compiler.builder.build_unconditional_branch(body_block);

    // describe condition
    compiler.builder.position_at_end(condition_block);
    let condition = condition(compiler)?;
    let get_boolean_fn = compiler.predefined_functions()?.get_boolean();
    let convert_to_boolean = compiler.predefined_functions()?.convert_to_boolean();
    let condition = convert_to_boolean.call(compiler, &condition);
    let condition = get_boolean_fn.call(compiler, &condition);
    let condition = compiler.builder.build_int_compare(
        inkwell::IntPredicate::EQ,
        condition,
        compiler.context.i8_type().const_int(1_u64, false),
        "",
    );
    compiler
        .builder
        .build_conditional_branch(condition, body_block, continue_block);

    // describe body
    compiler.builder.position_at_end(body_block);
    let mut is_returned = false;
    for expr in body {
        let is_return = expr.compile(compiler)?;
        if is_return {
            is_returned = true;
            break;
        }
    }
    if !is_returned {
        compiler.builder.build_unconditional_branch(condition_block);
    }

    //
    compiler.builder.position_at_end(continue_block);

    Ok(())
}
