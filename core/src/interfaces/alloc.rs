use crate::variable::Variable;

#[no_mangle]
pub unsafe extern "C" fn allocate() -> *mut Variable {
    Box::into_raw(Box::new(Variable::Undefined))
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(_this: *mut Variable) {}
