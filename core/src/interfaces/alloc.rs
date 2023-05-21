use crate::{ptr::RawPtr, variable::VariableValue};

#[no_mangle]
pub extern "C" fn allocate() -> *mut VariableValue {
    let val = VariableValue::Undefined;
    RawPtr::from(val).get_raw()
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(_this: *mut VariableValue) {}
