use crate::{
    array::Array,
    function::{FuncType, Function},
    number::Number,
    object::Object,
    ptr::RawPtr,
    variable::VariableValue,
};
use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn set_undefined(this: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Undefined;
}

#[no_mangle]
pub extern "C" fn set_null(this: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Null;
}

#[no_mangle]
pub extern "C" fn set_nan(this: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Number(Number::NaN);
}

#[no_mangle]
pub extern "C" fn set_infinity(this: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Number(Number::Infinity);
}

#[no_mangle]
pub extern "C" fn set_neginfinity(this: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Number(Number::NegInfinity);
}

#[no_mangle]
pub extern "C" fn set_number(this: *mut VariableValue, val: f64) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Number(Number::Number(val));
}

#[no_mangle]
pub extern "C" fn set_boolean(this: *mut VariableValue, val: bool) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Boolean(val);
}

#[no_mangle]
pub unsafe extern "C" fn set_string(this: *mut VariableValue, val: *const c_char) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let val = CStr::from_ptr(val).to_str().unwrap().to_string();

    *this = VariableValue::String(val);
}

#[no_mangle]
pub unsafe extern "C" fn set_empty_object(this: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Object(Object::new());
}

#[no_mangle]
pub unsafe extern "C" fn set_empty_array(this: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Array(Array::new());
}

#[no_mangle]
pub unsafe extern "C" fn set_function(this: *mut VariableValue, func: FuncType, args_num: u32) {
    let mut this = RawPtr::from_raw(this).unwrap();

    *this = VariableValue::Function(Function::new(func, args_num));
}

#[no_mangle]
pub extern "C" fn set_variable(this: *mut VariableValue, val: *mut VariableValue) {
    let mut this = RawPtr::from_raw(this).unwrap();
    let val = RawPtr::from_raw(val).unwrap();

    *this = (*val).clone();
}

#[no_mangle]
pub extern "C" fn get_boolean(this: *mut VariableValue) -> bool {
    let this = RawPtr::from_raw(this).unwrap();

    this.to_boolean()
}
