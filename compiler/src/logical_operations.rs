use crate::{variable::Field, Compiler, Error, Function, Variable};

pub fn logical_not<'ctx, T>(
    compiler: &mut Compiler<'ctx, T>,
    cur_function: &Function<'ctx, T>,
    arg: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    let arg = Variable::new_variable(compiler, cur_function, &arg);
    let number_case_f = |compiler: &Compiler<'ctx, T>| {
        let number_field = arg.get_field(compiler, Field::Number);
        let number_field = compiler
            .builder
            .build_load(number_field, "")
            .into_float_value();

        let true_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let false_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let continue_block = compiler
            .context
            .append_basic_block(cur_function.function, "");

        let comparison = compiler.builder.build_float_compare(
            inkwell::FloatPredicate::OEQ,
            number_field,
            compiler.context.f64_type().const_float(0_f64),
            "",
        );

        compiler
            .builder
            .build_conditional_branch(comparison, true_block, false_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        arg.assign_boolean(compiler, false);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe true case
        compiler.builder.position_at_end(true_block);
        arg.assign_boolean(compiler, true);
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    };
    let string_case_f = |_compiler: &Compiler<'ctx, T>| {
        // TODO implement
    };
    let boolean_case_f = |compiler: &Compiler<'ctx, T>| {
        let boolean_field = arg.get_field(compiler, Field::Boolean);
        let boolean_field = compiler
            .builder
            .build_load(boolean_field, "")
            .into_int_value();

        let true_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let false_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let continue_block = compiler
            .context
            .append_basic_block(cur_function.function, "");

        compiler
            .builder
            .build_conditional_branch(boolean_field, true_block, false_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        arg.assign_boolean(compiler, true);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe true case
        compiler.builder.position_at_end(true_block);
        arg.assign_boolean(compiler, false);
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    };

    arg.switch_type(
        compiler,
        cur_function,
        number_case_f,
        string_case_f,
        boolean_case_f,
    );
    Ok(arg)
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
