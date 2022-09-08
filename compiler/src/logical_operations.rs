use crate::{
    variable::{BooleanField, NumberField, StringField},
    Compiler, Error, Function, Variable,
};
use inkwell::{FloatPredicate, IntPredicate};

pub fn logical_not<'ctx, T>(
    compiler: &Compiler<'ctx, T>,
    cur_function: &Function<'ctx, T>,
    arg: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    let strlen_fn = compiler.predefined_functions().get_strlen()?;
    let arg = Variable::new_variable(compiler, cur_function, &arg);
    let number_case_f = |compiler: &Compiler<'ctx, T>| {
        let number_field = arg.get_field::<T, NumberField>(compiler);

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
            number_field.load_value(compiler),
            compiler.context.f64_type().const_float(0_f64),
            "",
        );

        compiler
            .builder
            .build_conditional_branch(comparison, true_block, false_block);

        // describe true case
        compiler.builder.position_at_end(true_block);
        arg.assign_boolean(compiler, true);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        arg.assign_boolean(compiler, false);
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    };
    let string_case_f = |_compiler: &Compiler<'ctx, T>| {
        let string_field = arg.get_field::<T, StringField>(compiler);
        let str_len = strlen_fn.call(compiler, string_field);

        let true_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let false_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let continue_block = compiler
            .context
            .append_basic_block(cur_function.function, "");

        let comparison = compiler.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            str_len,
            compiler.context.i32_type().const_int(0, false),
            "",
        );

        compiler
            .builder
            .build_conditional_branch(comparison, true_block, false_block);

        // describe true case
        compiler.builder.position_at_end(true_block);
        arg.assign_boolean(compiler, true);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        arg.assign_boolean(compiler, false);
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    };
    let boolean_case_f = |compiler: &Compiler<'ctx, T>| {
        let boolean_field = arg.get_field::<T, BooleanField>(compiler);

        let true_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let false_block = compiler
            .context
            .append_basic_block(cur_function.function, "");
        let continue_block = compiler
            .context
            .append_basic_block(cur_function.function, "");

        compiler.builder.build_conditional_branch(
            boolean_field.load_value(compiler),
            true_block,
            false_block,
        );

        // describe true case
        compiler.builder.position_at_end(true_block);
        arg.assign_boolean(compiler, false);
        compiler.builder.build_unconditional_branch(continue_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        arg.assign_boolean(compiler, true);
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
    compiler: &Compiler<'ctx, T>,
    cur_function: &Function<'ctx, T>,
    arg1: Variable<'ctx>,
    arg2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    let res = Variable::new(compiler);
    let strcmp_fn = compiler.predefined_functions().get_strcmp()?;
    // number case
    let arg1_number_case_f = |_compiler: &Compiler<'ctx, T>| {
        let arg2_number_case_f = |compiler: &Compiler<'ctx, T>| {
            let arg1_number_field = arg1.get_field::<T, NumberField>(compiler);
            let arg2_number_field = arg2.get_field::<T, NumberField>(compiler);

            let true_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let false_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let continue_block = compiler
                .context
                .append_basic_block(cur_function.function, "");

            compiler.builder.build_conditional_branch(
                compiler.builder.build_float_compare(
                    FloatPredicate::OEQ,
                    arg1_number_field.load_value(compiler),
                    arg2_number_field.load_value(compiler),
                    "",
                ),
                true_block,
                false_block,
            );

            // describe true case
            compiler.builder.position_at_end(true_block);
            res.assign_boolean(compiler, true);
            compiler.builder.build_unconditional_branch(continue_block);

            // describe false case
            compiler.builder.position_at_end(false_block);
            res.assign_boolean(compiler, false);
            compiler.builder.build_unconditional_branch(continue_block);

            //
            compiler.builder.position_at_end(continue_block);
        };
        let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, false);
        };
        let arg2_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, false);
        };
        arg2.switch_type(
            compiler,
            cur_function,
            arg2_number_case_f,
            arg2_string_case_f,
            arg2_boolean_case_f,
        );
    };

    // string case
    let arg1_string_case_f = |_compiler: &Compiler<'ctx, T>| {
        let arg2_number_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, false);
        };
        let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
            let arg1_string_field = arg1.get_field::<T, StringField>(compiler);
            let arg2_string_field = arg2.get_field::<T, StringField>(compiler);

            let ret = strcmp_fn.call(compiler, arg1_string_field, arg2_string_field);

            let true_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let false_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let continue_block = compiler
                .context
                .append_basic_block(cur_function.function, "");

            compiler.builder.build_conditional_branch(
                compiler.builder.build_int_compare(
                    IntPredicate::EQ,
                    ret,
                    compiler.context.i32_type().const_int(0, false),
                    "",
                ),
                true_block,
                false_block,
            );

            // describe true case
            compiler.builder.position_at_end(true_block);
            res.assign_boolean(compiler, true);
            compiler.builder.build_unconditional_branch(continue_block);

            // describe false case
            compiler.builder.position_at_end(false_block);
            res.assign_boolean(compiler, false);
            compiler.builder.build_unconditional_branch(continue_block);

            //
            compiler.builder.position_at_end(continue_block);
        };
        let arg2_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, false);
        };
        arg2.switch_type(
            compiler,
            cur_function,
            arg2_number_case_f,
            arg2_string_case_f,
            arg2_boolean_case_f,
        );
    };

    // boolean case
    let arg1_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
        let arg2_number_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, false);
        };
        let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, false);
        };
        let arg2_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            let arg1_boolean_field = arg1.get_field::<T, BooleanField>(compiler);
            let arg2_boolean_field = arg2.get_field::<T, BooleanField>(compiler);

            let true_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let false_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let continue_block = compiler
                .context
                .append_basic_block(cur_function.function, "");

            compiler.builder.build_conditional_branch(
                compiler.builder.build_int_compare(
                    IntPredicate::EQ,
                    arg1_boolean_field.load_value(compiler),
                    arg2_boolean_field.load_value(compiler),
                    "",
                ),
                true_block,
                false_block,
            );

            // describe true case
            compiler.builder.position_at_end(true_block);
            res.assign_boolean(compiler, true);
            compiler.builder.build_unconditional_branch(continue_block);

            // describe false case
            compiler.builder.position_at_end(false_block);
            res.assign_boolean(compiler, false);
            compiler.builder.build_unconditional_branch(continue_block);

            //
            compiler.builder.position_at_end(continue_block);
        };
        arg2.switch_type(
            compiler,
            cur_function,
            arg2_number_case_f,
            arg2_string_case_f,
            arg2_boolean_case_f,
        );
    };

    arg1.switch_type(
        compiler,
        cur_function,
        arg1_number_case_f,
        arg1_string_case_f,
        arg1_boolean_case_f,
    );
    Ok(res)
}

pub fn logical_sne<'ctx, T>(
    compiler: &Compiler<'ctx, T>,
    cur_function: &Function<'ctx, T>,
    arg1: Variable<'ctx>,
    arg2: Variable<'ctx>,
) -> Result<Variable<'ctx>, Error<T>> {
    let res = Variable::new(compiler);
    let strcmp_fn = compiler.predefined_functions().get_strcmp()?;
    // number case
    let arg1_number_case_f = |_compiler: &Compiler<'ctx, T>| {
        let arg2_number_case_f = |compiler: &Compiler<'ctx, T>| {
            let arg1_number_field = arg1.get_field::<T, NumberField>(compiler);
            let arg2_number_field = arg2.get_field::<T, NumberField>(compiler);

            let true_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let false_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let continue_block = compiler
                .context
                .append_basic_block(cur_function.function, "");

            compiler.builder.build_conditional_branch(
                compiler.builder.build_float_compare(
                    FloatPredicate::OEQ,
                    arg1_number_field.load_value(compiler),
                    arg2_number_field.load_value(compiler),
                    "",
                ),
                true_block,
                false_block,
            );

            // describe true case
            compiler.builder.position_at_end(true_block);
            res.assign_boolean(compiler, false);
            compiler.builder.build_unconditional_branch(continue_block);

            // describe false case
            compiler.builder.position_at_end(false_block);
            res.assign_boolean(compiler, true);
            compiler.builder.build_unconditional_branch(continue_block);

            //
            compiler.builder.position_at_end(continue_block);
        };
        let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, true);
        };
        let arg2_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, true);
        };
        arg2.switch_type(
            compiler,
            cur_function,
            arg2_number_case_f,
            arg2_string_case_f,
            arg2_boolean_case_f,
        );
    };

    // string case
    let arg1_string_case_f = |_compiler: &Compiler<'ctx, T>| {
        let arg2_number_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, true);
        };
        let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
            let arg1_string_field = arg1.get_field::<T, StringField>(compiler);
            let arg2_string_field = arg2.get_field::<T, StringField>(compiler);

            let ret = strcmp_fn.call(compiler, arg1_string_field, arg2_string_field);

            let true_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let false_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let continue_block = compiler
                .context
                .append_basic_block(cur_function.function, "");

            compiler.builder.build_conditional_branch(
                compiler.builder.build_int_compare(
                    IntPredicate::EQ,
                    ret,
                    compiler.context.i32_type().const_int(0, false),
                    "",
                ),
                true_block,
                false_block,
            );

            // describe true case
            compiler.builder.position_at_end(true_block);
            res.assign_boolean(compiler, false);
            compiler.builder.build_unconditional_branch(continue_block);

            // describe false case
            compiler.builder.position_at_end(false_block);
            res.assign_boolean(compiler, true);
            compiler.builder.build_unconditional_branch(continue_block);

            //
            compiler.builder.position_at_end(continue_block);
        };
        let arg2_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, true);
        };
        arg2.switch_type(
            compiler,
            cur_function,
            arg2_number_case_f,
            arg2_string_case_f,
            arg2_boolean_case_f,
        );
    };

    // boolean case
    let arg1_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
        let arg2_number_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, true);
        };
        let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
            res.assign_boolean(compiler, true);
        };
        let arg2_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            let arg1_boolean_field = arg1.get_field::<T, BooleanField>(compiler);
            let arg2_boolean_field = arg2.get_field::<T, BooleanField>(compiler);

            let true_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let false_block = compiler
                .context
                .append_basic_block(cur_function.function, "");
            let continue_block = compiler
                .context
                .append_basic_block(cur_function.function, "");

            compiler.builder.build_conditional_branch(
                compiler.builder.build_int_compare(
                    IntPredicate::EQ,
                    arg1_boolean_field.load_value(compiler),
                    arg2_boolean_field.load_value(compiler),
                    "",
                ),
                true_block,
                false_block,
            );

            // describe true case
            compiler.builder.position_at_end(true_block);
            res.assign_boolean(compiler, false);
            compiler.builder.build_unconditional_branch(continue_block);

            // describe false case
            compiler.builder.position_at_end(false_block);
            res.assign_boolean(compiler, true);
            compiler.builder.build_unconditional_branch(continue_block);

            //
            compiler.builder.position_at_end(continue_block);
        };
        arg2.switch_type(
            compiler,
            cur_function,
            arg2_number_case_f,
            arg2_string_case_f,
            arg2_boolean_case_f,
        );
    };

    arg1.switch_type(
        compiler,
        cur_function,
        arg1_number_case_f,
        arg1_string_case_f,
        arg1_boolean_case_f,
    );
    Ok(res)
}
