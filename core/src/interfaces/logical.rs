use crate::variable::Variable;
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn logical_not(val: *mut Variable) -> *mut Variable {
    let val = NonNull::new(val).unwrap();

    let res = val.as_ref().not();
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub unsafe extern "C" fn logical_and(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = NonNull::new(val1).unwrap();
    let val2 = NonNull::new(val2).unwrap();

    let res = Variable::and(val1.as_ref(), val2.as_ref());
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub unsafe extern "C" fn logical_or(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = NonNull::new(val1).unwrap();
    let val2 = NonNull::new(val2).unwrap();

    let res = Variable::or(val1.as_ref(), val2.as_ref());
    Box::into_raw(Box::new(res))
}
