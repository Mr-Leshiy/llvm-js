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
    let convert_to_boolean = compiler.predefined_functions()?.convert_to_boolean();
    let boolean_condition = convert_to_boolean.call(compiler, &condition);
    if condition.is_tmp() {
        condition.deallocate(compiler)?;
    }
    let int_val = get_boolean_fn.call(compiler, &boolean_condition);
    if boolean_condition.is_tmp() {
        boolean_condition.deallocate(compiler)?;
    }
    let int_val = compiler.builder.build_int_compare(
        inkwell::IntPredicate::EQ,
        int_val,
        compiler.context.i8_type().const_int(1_u64, false),
        "",
    );
    compiler
        .builder
        .build_conditional_branch(int_val, body_block, continue_block);

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
    let boolean_condition = convert_to_boolean.call(compiler, &condition);
    if condition.is_tmp() {
        condition.deallocate(compiler)?;
    }
    let int_val = get_boolean_fn.call(compiler, &boolean_condition);
    if boolean_condition.is_tmp() {
        boolean_condition.deallocate(compiler)?;
    }
    let int_val = compiler.builder.build_int_compare(
        inkwell::IntPredicate::EQ,
        int_val,
        compiler.context.i8_type().const_int(1_u64, false),
        "",
    );
    compiler
        .builder
        .build_conditional_branch(int_val, body_block, continue_block);

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
