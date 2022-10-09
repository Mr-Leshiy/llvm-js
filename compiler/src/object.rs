 
use inkwell::values::StructValue;

#[derive(Clone)]
pub struct Object<'ctx> {
    pub(crate) value: StructValue<'ctx>,
}

impl<'ctx> Object<'ctx> {
    pub const TYPE_NAME: &'static str = "Object";
}
