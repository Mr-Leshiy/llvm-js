use super::Compiler;
use crate::{InkwellContext, Variable};
use inkwell::{module::Linkage, values::FunctionValue, AddressSpace};

#[derive(Clone)]
pub struct AddPropertyByStrFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> AddPropertyByStrFn<'ctx> {
    const NAME: &'static str = "add_property_by_str";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let string_type = inkwell_context
            .context
            .i8_type()
            .ptr_type(AddressSpace::from(0));
        let function_type = inkwell_context.context.void_type().fn_type(
            &[var_type.into(), string_type.into(), var_type.into()],
            false,
        );
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &str,
        value: &Variable<'ctx>,
    ) {
        let key = compiler
            .inkwell_context
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[val.value.into(), key.into(), value.value.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct AddPropertyByVarFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> AddPropertyByVarFn<'ctx> {
    const NAME: &'static str = "add_property_by_var";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), var_type.into(), var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    #[allow(dead_code)]
    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &Variable<'ctx>,
        value: &Variable<'ctx>,
    ) {
        compiler.inkwell_context.builder.build_call(
            self.func,
            &[val.value.into(), key.value.into(), value.value.into()],
            "",
        );
    }
}

#[derive(Clone)]
pub struct GetPropertyByStrFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> GetPropertyByStrFn<'ctx> {
    const NAME: &'static str = "get_property_by_str";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let string_type = inkwell_context
            .context
            .i8_type()
            .ptr_type(AddressSpace::from(0));
        let function_type = var_type.fn_type(&[var_type.into(), string_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &str,
    ) -> Variable<'ctx> {
        let key = compiler
            .inkwell_context
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        let value = compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into(), key.into()], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable {
            value,
            is_tmp: false,
        }
    }
}

#[derive(Clone)]
pub struct GetPropertyByVarFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> GetPropertyByVarFn<'ctx> {
    const NAME: &'static str = "get_property_by_var";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let function_type = var_type.fn_type(&[var_type.into(), var_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        key: &Variable<'ctx>,
    ) -> Variable<'ctx> {
        let value = compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into(), key.value.into()], "")
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable {
            value,
            is_tmp: false,
        }
    }
}

#[derive(Clone)]
pub struct RemovePropertyFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> RemovePropertyFn<'ctx> {
    const NAME: &'static str = "remove_property";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let string_type = inkwell_context
            .context
            .i8_type()
            .ptr_type(AddressSpace::from(0));
        let function_type = inkwell_context
            .context
            .void_type()
            .fn_type(&[var_type.into(), string_type.into()], false);
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(&self, compiler: &Compiler<'ctx, T>, val: &Variable<'ctx>, key: &str) {
        let key = compiler
            .inkwell_context
            .builder
            .build_global_string_ptr(key, "")
            .as_pointer_value();
        compiler
            .inkwell_context
            .builder
            .build_call(self.func, &[val.value.into(), key.into()], "");
    }
}

#[derive(Clone)]
pub struct FunctionCallFn<'ctx> {
    func: FunctionValue<'ctx>,
}

impl<'ctx> FunctionCallFn<'ctx> {
    const NAME: &'static str = "function_call";

    pub(super) fn declare(inkwell_context: &InkwellContext<'ctx>) -> Self {
        let var_type = inkwell_context.variable_type;
        let u32_type = inkwell_context.context.i32_type();
        let function_type = var_type.fn_type(
            &[
                var_type.into(),
                var_type.ptr_type(AddressSpace::from(0)).into(),
                u32_type.into(),
            ],
            false,
        );
        let func =
            inkwell_context
                .module
                .add_function(Self::NAME, function_type, Some(Linkage::External));
        Self { func }
    }

    pub(crate) fn call<T>(
        &self,
        compiler: &Compiler<'ctx, T>,
        val: &Variable<'ctx>,
        args: &[Variable<'ctx>],
    ) -> Variable<'ctx> {
        let var_type = compiler.inkwell_context.variable_type;

        let array = compiler
            .inkwell_context
            .builder
            .build_alloca(var_type.array_type(args.len().try_into().unwrap()), "");
        let args_len = compiler
            .inkwell_context
            .context
            .i32_type()
            .const_int(args.len().try_into().unwrap(), false);

        for (i, arg) in args.iter().enumerate() {
            unsafe {
                let ptr = compiler.inkwell_context.builder.build_gep(
                    array,
                    &[
                        compiler
                            .inkwell_context
                            .context
                            .i32_type()
                            .const_int(0, false),
                        compiler
                            .inkwell_context
                            .context
                            .i32_type()
                            .const_int(i.try_into().unwrap(), false),
                    ],
                    "",
                );
                let ptr = compiler
                    .inkwell_context
                    .builder
                    .build_bitcast(ptr, var_type.ptr_type(AddressSpace::from(0)), "")
                    .into_pointer_value();
                compiler.inkwell_context.builder.build_store(ptr, arg.value);
            }
        }

        let args = compiler.inkwell_context.builder.build_bitcast(
            array,
            var_type.ptr_type(AddressSpace::from(0)),
            "",
        );

        let value = compiler
            .inkwell_context
            .builder
            .build_call(
                self.func,
                &[val.value.into(), args.into(), args_len.into()],
                "",
            )
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();
        Variable {
            value,
            is_tmp: true,
        }
    }
}
