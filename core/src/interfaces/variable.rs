use crate::{number::Number, object::Object, pointer::Ptr, variable::Variable};
use std::ffi::{c_char, CStr};

#[no_mangle]
pub extern "C" fn set_undefined(this: *mut Variable) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Undefined;
}

#[no_mangle]
pub extern "C" fn set_null(this: *mut Variable) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Null;
}

#[no_mangle]
pub extern "C" fn set_nan(this: *mut Variable) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Number(Number::NaN);
}

#[no_mangle]
pub extern "C" fn set_infinity(this: *mut Variable) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Number(Number::Infinity);
}

#[no_mangle]
pub extern "C" fn set_neginfinity(this: *mut Variable) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Number(Number::NegInfinity);
}

#[no_mangle]
pub extern "C" fn set_number(this: *mut Variable, val: f64) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Number(Number::Number(val));
}

#[no_mangle]
pub extern "C" fn set_boolean(this: *mut Variable, val: bool) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Boolean(val);
}

#[no_mangle]
pub unsafe extern "C" fn set_string(this: *mut Variable, val: *const c_char) {
    let mut this = Ptr::from_raw(this).unwrap();
    let val = CStr::from_ptr(val).to_str().unwrap().to_string();

    *this.get_mut_ref() = Variable::String(val);
}

#[no_mangle]
pub unsafe extern "C" fn set_empty_object(this: *mut Variable) {
    let mut this = Ptr::from_raw(this).unwrap();

    *this.get_mut_ref() = Variable::Object(Object::new_object());
}

#[no_mangle]
pub extern "C" fn set_variable(this: *mut Variable, val: *mut Variable) {
    let mut this = Ptr::from_raw(this).unwrap();
    let val = Ptr::from_raw(val).unwrap();

    *this.get_mut_ref() = val.get_ref().clone();
}

#[no_mangle]
pub extern "C" fn get_boolean(this: *mut Variable) -> bool {
    let this = Ptr::from_raw(this).unwrap();

    this.get_ref().to_boolean()
}
