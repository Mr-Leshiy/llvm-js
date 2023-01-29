use crate::{Compile, Compiler, Error, Variable};

pub fn generate_if_else<'ctx, T, Expr: Compile<T, Output = bool>>(
    compiler: &mut Compiler<'ctx, T>,
    condition: &Variable<'ctx>,
    true_case_body: Vec<Expr>,
    else_case_body: Vec<Expr>,
) -> Result<bool, Error<T>> {
    let get_boolean_fn = compiler.predefined_functions()?.get_boolean();

    let condition_boolean = get_boolean_fn.call(compiler, condition);
    if condition.is_tmp() {
        condition.deallocate(compiler)?;
    }

    let condition_boolean = compiler.builder.build_int_compare(
        inkwell::IntPredicate::EQ,
        condition_boolean,
        compiler.context.i8_type().const_int(1_u64, false),
        "",
    );

    let true_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");
    let false_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");
    let continue_block = compiler
        .context
        .append_basic_block(compiler.cur_function.as_ref().unwrap().function, "");

    compiler
        .builder
        .build_conditional_branch(condition_boolean, true_block, false_block);

    // describe true case
    compiler.builder.position_at_end(true_block);
    let mut is_true_returned = false;
    for expr in true_case_body {
        let is_return = expr.compile(compiler)?;
        if is_return {
            is_true_returned = true;
            break;
        }
    }
    if !is_true_returned {
        compiler.builder.build_unconditional_branch(continue_block);
    }

    // describe false case
    compiler.builder.position_at_end(false_block);
    let mut is_else_returned = false;
    for expr in else_case_body {
        let is_return = expr.compile(compiler)?;
        if is_return {
            is_else_returned = true;
            break;
        }
    }
    if !is_else_returned {
        compiler.builder.build_unconditional_branch(continue_block);
    }

    //
    compiler.builder.position_at_end(continue_block);
    Ok(false)
}
