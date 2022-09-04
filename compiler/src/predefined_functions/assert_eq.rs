use super::{abort::AbortFn, Compiler, PredefineFunctionName};
use crate::{
    variable::{BooleanField, NumberField},
    Error, Function, Variable,
};
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
    pub fn assert_eq<'ctx, T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        cur_function: &Function<'ctx, T>,
        abort_fn: &AbortFn<'ctx>,
        arg1: Variable<'ctx>,
        arg2: Variable<'ctx>,
    ) -> Result<(), Error<T>> {
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

                // describe false case
                compiler.builder.position_at_end(false_block);
                abort_fn.abort(compiler);
                compiler.builder.build_unconditional_branch(true_block);

                // describe true case
                compiler.builder.position_at_end(true_block);
            };
            let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
                abort_fn.abort(compiler);
            };
            let arg2_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
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
        let arg1_string_case_f = |_compiler: &Compiler<'ctx, T>| {
            // TODO implement
            abort_fn.abort(compiler);
        };

        // boolean case
        let arg1_boolean_case_f = |compiler: &Compiler<'ctx, T>| {
            let arg2_number_case_f = |compiler: &Compiler<'ctx, T>| {
                abort_fn.abort(compiler);
            };
            let arg2_string_case_f = |compiler: &Compiler<'ctx, T>| {
                abort_fn.abort(compiler);
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
