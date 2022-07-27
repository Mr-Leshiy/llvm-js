use super::{abort::AbortFn, Compiler, PredefineFunctionName};
use crate::{variable::Field, Error, Function, Variable, VariableValue};
use inkwell::{FloatPredicate, IntPredicate};

#[derive(Clone)]
pub struct AssertEqFn;

impl PredefineFunctionName for AssertEqFn {
    const NAME: &'static str = "assert_eq";
}

impl AssertEqFn {
    pub(super) fn declare() -> Self {
        Self
    }
}

impl AssertEqFn {
    pub fn assert_eq<'ctx>(
        &self,
        compiler: &Compiler<'ctx>,
        cur_function: &Function<'ctx>,
        abort_fn: &AbortFn<'ctx>,
        arg1: VariableValue,
        arg2: VariableValue,
    ) -> Result<(), Error> {
        let arg1 = Variable::try_from_variable_value(compiler, cur_function, arg1)?;
        let arg2 = Variable::try_from_variable_value(compiler, cur_function, arg2)?;

        // number case
        let arg1_number_case_f = |_compiler: &Compiler<'ctx>| {
            let arg2_number_case_f = |compiler: &Compiler<'ctx>| {
                let arg1_number_field = arg1.get_field(compiler, Field::Number);
                let arg1_number_field = compiler
                    .builder
                    .build_load(arg1_number_field, "")
                    .into_float_value();

                let arg2_number_field = arg2.get_field(compiler, Field::Number);
                let arg2_number_field = compiler
                    .builder
                    .build_load(arg2_number_field, "")
                    .into_float_value();

                let true_block = compiler
                    .context
                    .append_basic_block(cur_function.function, "");
                let false_block = compiler
                    .context
                    .append_basic_block(cur_function.function, "");

                compiler.builder.build_conditional_branch(
                    compiler.builder.build_float_compare(
                        FloatPredicate::OEQ,
                        arg1_number_field,
                        arg2_number_field,
                        "",
                    ),
                    true_block,
                    false_block,
                );

                // describe false case
                compiler.builder.position_at_end(false_block);
                abort_fn.abort(compiler);
                compiler.builder.build_unconditional_branch(true_block);

                // describe true case
                compiler.builder.position_at_end(true_block);
            };
            let arg2_string_case_f = |compiler: &Compiler<'ctx>| {
                abort_fn.abort(compiler);
            };
            let arg2_boolean_case_f = |compiler: &Compiler<'ctx>| {
                abort_fn.abort(compiler);
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
        let arg1_string_case_f = |_compiler: &Compiler<'ctx>| {
            // TODO implement
            abort_fn.abort(compiler);
        };

        // boolean case
        let arg1_boolean_case_f = |compiler: &Compiler<'ctx>| {
            let arg2_number_case_f = |compiler: &Compiler<'ctx>| {
                abort_fn.abort(compiler);
            };
            let arg2_string_case_f = |compiler: &Compiler<'ctx>| {
                abort_fn.abort(compiler);
            };
            let arg2_boolean_case_f = |compiler: &Compiler<'ctx>| {
                let arg1_boolean_field = arg1.get_field(compiler, Field::Boolean);
                let arg1_boolean_field = compiler
                    .builder
                    .build_load(arg1_boolean_field, "")
                    .into_int_value();

                let arg2_boolean_field = arg2.get_field(compiler, Field::Boolean);
                let arg2_boolean_field = compiler
                    .builder
                    .build_load(arg2_boolean_field, "")
                    .into_int_value();

                let true_block = compiler
                    .context
                    .append_basic_block(cur_function.function, "");
                let false_block = compiler
                    .context
                    .append_basic_block(cur_function.function, "");

                compiler.builder.build_conditional_branch(
                    compiler.builder.build_int_compare(
                        IntPredicate::EQ,
                        arg1_boolean_field,
                        arg2_boolean_field,
                        "",
                    ),
                    true_block,
                    false_block,
                );

                // describe false case
                compiler.builder.position_at_end(false_block);
                abort_fn.abort(compiler);
                compiler.builder.build_unconditional_branch(true_block);

                // describe true case
                compiler.builder.position_at_end(true_block);
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
        Ok(())
    }
}
