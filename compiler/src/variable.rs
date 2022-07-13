use super::Compiler;
use inkwell::{types::StructType, values::PointerValue, AddressSpace};

pub enum Type {
    Number = 0,
    String = 1,
}

#[derive(Clone)]
pub struct Variable<'ctx> {
    pub(super) value: PointerValue<'ctx>,
}

impl<'ctx> Variable<'ctx> {
    fn get_type(compiler: &mut Compiler<'ctx>) -> StructType<'ctx> {
        let number_type = compiler.context.f64_type();
        let string_type = compiler.context.i8_type().ptr_type(AddressSpace::Generic);

        compiler
            .context
            .struct_type(&[number_type.into(), string_type.into()], false)
    }

    fn new(compiler: &mut Compiler<'ctx>, name: &str) -> Self {
        let var_type = Self::get_type(compiler);

        let value = compiler.builder.build_alloca(var_type, name);
        Self { value }
    }
}

impl<'ctx> Variable<'ctx> {
    pub fn new_number(compiler: &mut Compiler<'ctx>, number: f64, name: &str) -> Self {
        let variable = Self::new(compiler, name);
        variable.assign_number(compiler, number);
        variable
    }

    pub fn assign_number(&self, compiler: &mut Compiler<'ctx>, number: f64) {
        let number = compiler.context.f64_type().const_float(number);
        let number_field = compiler
            .builder
            .build_struct_gep(self.value, Type::Number as u32, "")
            .unwrap();
        compiler.builder.build_store(number_field, number);
    }

    pub fn new_string(compiler: &mut Compiler<'ctx>, string: &str, name: &str) -> Self {
        let variable = Self::new(compiler, name);
        variable.assign_string(compiler, string);
        variable
    }

    pub fn assign_string(&self, compiler: &mut Compiler<'ctx>, string: &str) {
        let string = compiler.context.const_string(string.as_bytes(), false);
        let string = string
            .get_element_as_constant(0)
            .into_int_value()
            .const_to_pointer(compiler.context.i8_type().ptr_type(AddressSpace::Generic));

        let string_field = compiler
            .builder
            .build_struct_gep(self.value, Type::String as u32, "")
            .unwrap();
        compiler.builder.build_store(string_field, string);
    }

    pub fn new_variable(compiler: &mut Compiler<'ctx>, name: &str, variable2: &Variable) -> Self {
        let variable1 = Self::new(compiler, name);
        variable1.assign_variable(compiler, variable2);
        variable1
    }

    pub fn assign_variable(&self, _compiler: &mut Compiler<'ctx>, _variable: &Variable) {
        // TODO: implement
    }
}
