use crate::{ptr::RawPtr, variable::Variable};

#[no_mangle]
pub unsafe extern "C" fn logical_not(val: *mut Variable) -> *mut Variable {
    let val = RawPtr::from_raw(val).unwrap();

    RawPtr::allocate(val.get_ref().not()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_and(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate(Variable::and(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_or(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate(Variable::or(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_eq(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate((val1.get_ref() == val2.get_ref()).into()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_ne(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate((val1.get_ref() != val2.get_ref()).into()).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_gt(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate(Variable::gt(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_ge(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate(Variable::ge(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_lt(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate(Variable::lt(val1.get_ref(), val2.get_ref())).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn logical_le(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = RawPtr::from_raw(val1).unwrap();
    let val2 = RawPtr::from_raw(val2).unwrap();

    RawPtr::allocate(Variable::le(val1.get_ref(), val2.get_ref())).get_raw()
}
