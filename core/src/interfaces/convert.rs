use crate::{ptr::RawPtr, variable::Variable};

#[no_mangle]
pub extern "C" fn convert_to_number(this: *mut Variable) -> *mut Variable {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(Variable::from(this.to_number())).get_raw()
}

#[no_mangle]
pub extern "C" fn convert_to_boolean(this: *mut Variable) -> *mut Variable {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(Variable::from(this.to_boolean())).get_raw()
}

#[no_mangle]
pub extern "C" fn convert_to_string(this: *mut Variable) -> *mut Variable {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(Variable::from(this.to_string())).get_raw()
}
