use crate::{pointer::Ptr, variable::Variable};

#[no_mangle]
pub unsafe extern "C" fn logical_not(val: *mut Variable) -> *mut Variable {
    let val = Ptr::from_raw(val).unwrap();

    Ptr::allocate(val.get_ref().not()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_and(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate(Variable::and(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_or(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate(Variable::or(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_eq(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate((val1.get_ref() == val2.get_ref()).into()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_ne(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate((val1.get_ref() != val2.get_ref()).into()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_gt(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate(Variable::gt(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_ge(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate(Variable::ge(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_lt(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate(Variable::lt(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_le(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    Ptr::allocate(Variable::le(val1.get_ref(), val2.get_ref())).get_raw()
}
