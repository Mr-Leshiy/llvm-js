use crate::{variable::Field, Compiler, Error, Function, Variable, VariableValue};

pub fn logical_not<'ctx>(
    compiler: &Compiler<'ctx>,
    cur_function: &Function<'ctx>,
    var: VariableValue,
) -> Result<Variable<'ctx>, Error> {
    // TODO optimize it

    let var = Variable::try_from_variable_value(compiler, cur_function, var)?;
    let mut res = var.clone();

    let arg_number_case_f = |_compiler: &Compiler<'ctx>| {
        // TODO implement
    };
    let arg_string_case_f = |_compiler: &Compiler<'ctx>| {
        // TODO implement
    };
    let arg_boolean_case_f = |compiler: &Compiler<'ctx>| {
        let boolean_field = var.get_field(compiler, Field::Boolean);

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

        // describe true case
        compiler.builder.position_at_end(true_block);
        res = Variable::new_boolean(compiler, true, "");
        compiler.builder.build_unconditional_branch(continue_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        res = Variable::new_boolean(compiler, false, "");
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    };

    var.switch_type(
        compiler,
        cur_function,
        arg_number_case_f,
        arg_string_case_f,
        arg_boolean_case_f,
    );
    Ok(res)
}

pub fn logical_and<'ctx>(
    compiler: &Compiler<'ctx>,
    cur_function: &Function<'ctx>,
    var1: VariableValue,
    var2: VariableValue,
) -> Result<Variable<'ctx>, Error> {
    // TODO optimize it

    let var1 = Variable::try_from_variable_value(compiler, cur_function, var1)?;
    let mut res = var1.clone();

    let arg_number_case_f = |_compiler: &Compiler<'ctx>| {
        // TODO implement
    };
    let arg_string_case_f = |_compiler: &Compiler<'ctx>| {
        // TODO implement
    };
    let arg_boolean_case_f = |compiler: &Compiler<'ctx>| {
        let boolean_field = var1.get_field(compiler, Field::Boolean);

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

        // describe true case
        compiler.builder.position_at_end(true_block);
        res = Variable::new_boolean(compiler, true, "");
        compiler.builder.build_unconditional_branch(continue_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        res = Variable::new_boolean(compiler, false, "");
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    };

    var1.switch_type(
        compiler,
        cur_function,
        arg_number_case_f,
        arg_string_case_f,
        arg_boolean_case_f,
    );
    Ok(res)
}

pub fn logical_or<'ctx>(
    compiler: &Compiler<'ctx>,
    cur_function: &Function<'ctx>,
    var1: VariableValue,
    var2: VariableValue,
) -> Result<Variable<'ctx>, Error> {
    // TODO optimize it

    let var1 = Variable::try_from_variable_value(compiler, cur_function, var1)?;
    let mut res = var1.clone();

    let arg_number_case_f = |_compiler: &Compiler<'ctx>| {
        // TODO implement
    };
    let arg_string_case_f = |_compiler: &Compiler<'ctx>| {
        // TODO implement
    };
    let arg_boolean_case_f = |compiler: &Compiler<'ctx>| {
        let boolean_field = var1.get_field(compiler, Field::Boolean);

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

        // describe true case
        compiler.builder.position_at_end(true_block);
        res = Variable::new_boolean(compiler, true, "");
        compiler.builder.build_unconditional_branch(continue_block);

        // describe false case
        compiler.builder.position_at_end(false_block);
        res = Variable::new_boolean(compiler, false, "");
        compiler.builder.build_unconditional_branch(continue_block);

        //
        compiler.builder.position_at_end(continue_block);
    };

    var1.switch_type(
        compiler,
        cur_function,
        arg_number_case_f,
        arg_string_case_f,
        arg_boolean_case_f,
    );
    Ok(res)
}
