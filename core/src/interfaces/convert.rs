use crate::{ptr::RawPtr, variable::Variable};

#[no_mangle]
pub extern "C" fn convert_to_number(this: *mut Variable) -> *mut Variable {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(this.get_ref().to_number().into()).get_raw()
}

#[no_mangle]
pub extern "C" fn convert_to_boolean(this: *mut Variable) -> *mut Variable {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(this.get_ref().to_boolean().into()).get_raw()
}

#[no_mangle]
pub extern "C" fn convert_to_string(this: *mut Variable) -> *mut Variable {
    let this = RawPtr::from_raw(this).unwrap();

    RawPtr::allocate(this.get_ref().to_string().into()).get_raw()
}
