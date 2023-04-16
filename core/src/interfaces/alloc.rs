use crate::variable::Variable;

#[no_mangle]
pub extern "C" fn allocate() -> *mut Variable {
    let res = Variable::Undefined;
    Box::into_raw(Box::new(res))
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(_this: *mut Variable) {}
