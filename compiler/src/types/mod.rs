pub use boolean::Boolean;
pub use float::Float;
use inkwell::values::PointerValue;
pub use integer::IntegerI8;
pub use string::String;

mod boolean;
mod float;
mod integer;
mod string;

pub trait ValueType<'ctx> {
    fn new(ptr: PointerValue<'ctx>) -> Self;
}
