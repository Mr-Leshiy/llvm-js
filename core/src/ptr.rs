use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct RawPtr<T> {
    raw: *mut T,
}

impl<T> RawPtr<T> {
    pub fn from_raw(raw: *mut T) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    pub fn copy(&self) -> Self {
        Self::from_raw(self.raw).expect("should be always valid")
    }

    pub fn get_raw(&self) -> *mut T {
        self.raw
    }
}

impl<T: Default> Default for RawPtr<T> {
    fn default() -> Self {
        Self::from(T::default())
    }
}

impl<T> From<T> for RawPtr<T> {
    fn from(val: T) -> Self {
        let ptr = Box::into_raw(Box::new(val));
        Self::from_raw(ptr).unwrap()
    }
}

impl<T> Clone for RawPtr<T> {
    fn clone(&self) -> Self {
        Self { raw: self.raw }
    }
}

impl<T: PartialEq> PartialEq for RawPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.raw) == (*other.raw) }
    }
}

impl<T> Deref for RawPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.raw }
    }
}

impl<T> DerefMut for RawPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.raw }
    }
}

#[derive(Debug)]
pub struct SmartPtr<T> {
    raw: *mut T,
}

impl<T> SmartPtr<T> {
    pub fn allocate(val: T) -> Self {
        let ptr = Box::into_raw(Box::new(val));
        Self::from_raw(ptr).unwrap()
    }

    pub fn from_raw(raw: *mut T) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }
}

impl<T: Default> Default for SmartPtr<T> {
    fn default() -> Self {
        Self::allocate(T::default())
    }
}

impl<T> Clone for SmartPtr<T> {
    fn clone(&self) -> Self {
        Self { raw: self.raw }
    }
}

impl<T: PartialEq> PartialEq for SmartPtr<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.raw) == (*other.raw) }
    }
}

impl<T> Deref for SmartPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.raw }
    }
}

impl<T> DerefMut for SmartPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.raw }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_ptr_eq_test() {
        assert_eq!(RawPtr::from(5), RawPtr::from(5));
        assert_ne!(RawPtr::from(5), RawPtr::from(10));
    }

    #[test]
    fn raw_ptr_test() {
        let mut ptr = RawPtr::from(5);

        assert_eq!(ptr.deref(), &5);
        assert_eq!(ptr.deref_mut(), &mut 5);
        unsafe {
            assert_eq!(*ptr.get_raw(), 5);
        }
    }
}
