use crate::variable::Variable;
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn arithmetic_addition(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = NonNull::new(val1).unwrap();
    let val2 = NonNull::new(val2).unwrap();

    Box::into_raw(Box::new(val1.as_ref() + val2.as_ref()))
}

#[no_mangle]
pub unsafe extern "C" fn arithmetic_substraction(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = NonNull::new(val1).unwrap();
    let val2 = NonNull::new(val2).unwrap();

    Box::into_raw(Box::new(val1.as_ref() - val2.as_ref()))
}

#[no_mangle]
pub unsafe extern "C" fn arithmetic_multiplication(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = NonNull::new(val1).unwrap();
    let val2 = NonNull::new(val2).unwrap();

    Box::into_raw(Box::new(val1.as_ref() * val2.as_ref()))
}

#[no_mangle]
pub unsafe extern "C" fn arithmetic_division(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = NonNull::new(val1).unwrap();
    let val2 = NonNull::new(val2).unwrap();

    Box::into_raw(Box::new(val1.as_ref() / val2.as_ref()))
}
