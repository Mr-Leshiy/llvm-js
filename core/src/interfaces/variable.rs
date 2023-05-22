use crate::{
    array::Array,
    function::{FuncType, Function},
    number::Number,
    object::Object,
    ptr::RawPtr,
    variable::{Variable, VariableValue},
};
use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn set_undefined(this: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Undefined;
}

#[no_mangle]
pub extern "C" fn set_null(this: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Null;
}

#[no_mangle]
pub extern "C" fn set_nan(this: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Number(Number::NaN);
}

#[no_mangle]
pub extern "C" fn set_infinity(this: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Number(Number::Infinity);
}

#[no_mangle]
pub extern "C" fn set_neginfinity(this: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Number(Number::NegInfinity);
}

#[no_mangle]
pub extern "C" fn set_number(this: *mut Variable, val: f64) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Number(Number::Number(val));
}

#[no_mangle]
pub extern "C" fn set_boolean(this: *mut Variable, val: bool) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Boolean(val);
}

#[no_mangle]
pub unsafe extern "C" fn set_string(this: *mut Variable, val: *const c_char) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let val = CStr::from_ptr(val).to_str().unwrap().to_string();

    **this = VariableValue::String(val);
}

#[no_mangle]
pub unsafe extern "C" fn set_empty_object(this: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Object(Object::new());
}

#[no_mangle]
pub unsafe extern "C" fn set_empty_array(this: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Array(Array::new());
}

#[no_mangle]
pub unsafe extern "C" fn set_function(this: *mut Variable, func: FuncType, args_num: u32) {
    let mut this = RawPtr::from_raw(this).unwrap();

    **this = VariableValue::Function(Function::new(func, args_num));
}

#[no_mangle]
pub extern "C" fn set_variable(this: *mut Variable, val: *mut Variable) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let val = RawPtr::from_raw(val).unwrap();
    println!("set_variable, {this:?}, {val:?}");

    **this = (**val).clone();
}

#[no_mangle]
pub extern "C" fn get_boolean(this: *mut Variable) -> bool {
    let this = RawPtr::from_raw(this).unwrap();

    this.to_boolean()
}
