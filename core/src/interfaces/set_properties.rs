use crate::{number::Number, variable::Variable};
use std::{
    ffi::{c_char, CStr},
    ptr::NonNull,
};

#[no_mangle]
pub unsafe extern "C" fn set_undefined(this: *mut Variable) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::Undefined;
}

#[no_mangle]
pub unsafe extern "C" fn set_null(this: *mut Variable) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::Null;
}

#[no_mangle]
pub unsafe extern "C" fn set_nan(this: *mut Variable) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::Number(Number::NaN);
}

#[no_mangle]
pub unsafe extern "C" fn set_infinity(this: *mut Variable) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::Number(Number::Infinity);
}

#[no_mangle]
pub unsafe extern "C" fn set_neginfinity(this: *mut Variable) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::Number(Number::NegInfinity);
}

#[no_mangle]
pub unsafe extern "C" fn set_number(this: *mut Variable, val: f64) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::Number(Number::Number(val));
}

#[no_mangle]
pub unsafe extern "C" fn set_boolean(this: *mut Variable, val: bool) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::Boolean(val);
}

#[no_mangle]
pub unsafe extern "C" fn set_string(this: *mut Variable, val: *const c_char) {
    let mut this = NonNull::new(this).unwrap();
    *this.as_mut() = Variable::String(CStr::from_ptr(val).to_str().unwrap().to_string());
}

#[no_mangle]
pub unsafe extern "C" fn set_variable(this: *mut Variable, val: *mut Variable) {
    let mut this = NonNull::new(this).unwrap();
    let val = NonNull::new(val).unwrap();
    *this.as_mut() = val.as_ref().clone();
}
