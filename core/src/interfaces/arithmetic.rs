use crate::{ptr::Ptr, variable::Variable};

#[no_mangle]
pub extern "C" fn arithmetic_addition(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    let res = Variable::add(val1.get_ref(), val2.get_ref());
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn arithmetic_substraction(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    let res = Variable::sub(val1.get_ref(), val2.get_ref());
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn arithmetic_multiplication(
    val1: *mut Variable,
    val2: *mut Variable,
) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    let res = Variable::mul(val1.get_ref(), val2.get_ref());
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub extern "C" fn arithmetic_division(val1: *mut Variable, val2: *mut Variable) -> *mut Variable {
    let val1 = Ptr::from_raw(val1).unwrap();
    let val2 = Ptr::from_raw(val2).unwrap();

    let res = Variable::div(val1.get_ref(), val2.get_ref());
    Box::into_raw(Box::new(res))
}
