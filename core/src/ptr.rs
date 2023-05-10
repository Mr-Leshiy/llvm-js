#[derive(Debug)]
pub struct Ptr<T> {
    raw: *mut T,
}

impl<T> Ptr<T> {
    pub fn copy(&self) -> Self {
        Self::from_raw(self.raw).expect("should be always valid")
    }

    pub fn from_raw(raw: *mut T) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self { raw })
        }
    }

    pub fn get_raw(&self) -> *mut T {
        self.raw
    }

    pub fn get_ref(&self) -> &T {
        unsafe { &*self.raw }
    }

    pub fn get_mut_ref(&mut self) -> &mut T {
        unsafe { &mut *self.raw }
    }
}

impl<T> Ptr<T> {
    pub fn allocate(val: T) -> Self {
        let ptr = Box::into_raw(Box::new(val));
        Self::from_raw(ptr).unwrap()
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Self { raw: self.raw }
    }
}

impl<T: PartialEq> PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.raw) == (*other.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test() {
        assert_eq!(Ptr::allocate(5), Ptr::allocate(5));
        assert_ne!(Ptr::allocate(5), Ptr::allocate(10));
    }

    #[test]
    fn basic_test() {
        let mut ptr = Ptr::allocate(5);

        assert_eq!(ptr.get_ref(), &5);
        assert_eq!(ptr.get_mut_ref(), &mut 5);
        unsafe {
            assert_eq!(*ptr.get_raw(), 5);
        }
    }
}
