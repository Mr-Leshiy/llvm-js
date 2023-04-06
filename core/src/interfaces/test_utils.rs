use crate::variable::Variable;
use std::{process::abort, ptr::NonNull};

#[no_mangle]
pub unsafe extern "C" fn variable_assert(this: *mut Variable) {
    let this = NonNull::new(this).unwrap();

    if !this.as_ref().to_boolean() {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn variable_assert_eq(val1: *mut Variable, val2: *mut Variable) {
    let val1 = NonNull::new(val1).unwrap();
    let val2 = NonNull::new(val2).unwrap();

    if val1.as_ref() != val2.as_ref() {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn print(this: *mut Variable) {
    let this = NonNull::new(this).unwrap();

    println!("{}", this.as_ref().to_string());
}
