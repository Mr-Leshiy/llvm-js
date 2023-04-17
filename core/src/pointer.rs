use std::marker::PhantomData;

pub type Ptr<T> = Pointer<T, ()>;

#[derive(Debug)]
pub struct Pointer<T, A> {
    raw: *mut T,
    _a: PhantomData<A>,
}

impl<T, A> Pointer<T, A> {
    pub fn copy(&self) -> Self {
        Self::from_raw(self.raw).expect("should be always valid")
    }

    pub fn from_raw(raw: *mut T) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self {
                raw,
                _a: PhantomData::default(),
            })
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

impl<T> Pointer<T, ()> {
    pub fn allocate(val: T) -> Self {
        Self::from_raw(Box::into_raw(Box::new(val))).unwrap()
    }
}

impl<T> Clone for Pointer<T, ()> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw,
            _a: self._a,
        }
    }
}

impl<T: PartialEq, A> PartialEq for Pointer<T, A> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.raw) == (*other.raw) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test() {
        assert_eq!(Pointer::allocate(5), Pointer::allocate(5));
        assert_ne!(Pointer::allocate(5), Pointer::allocate(10));
    }

    #[test]
    fn basic_test() {
        let mut ptr = Pointer::allocate(5);

        assert_eq!(ptr.get_ref(), &5);
        assert_eq!(ptr.get_mut_ref(), &mut 5);
        unsafe {
            assert_eq!(*ptr.get_raw(), 5);
        }
    }
}
