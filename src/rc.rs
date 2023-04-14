pub struct Rc<T>(*mut Inner<T>);

impl<T> Clone for Rc<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            self.0.as_mut().unwrap().1 += 1;
            Self(self.0)
        }
    }
}

impl<T> Rc<T> {
    pub fn new(val: T) -> Self {
        Self(Inner::new(val).into_ptr())
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.0.as_ref().unwrap().0 }
    }
}

impl<T> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

impl<T> Drop for Rc<T> {
    #[inline]
    fn drop(&mut self) {
        let v = unsafe { self.0.as_mut() }.unwrap();
        v.1 -= 1;
        if v.1 == 0 {
            unsafe { self.0.drop_in_place() };
        }
    }
}

struct Inner<T>(T, usize);

impl<T> Inner<T> {
    pub(super) fn new(val: T) -> Self {
        Self(val, 1)
    }

    #[inline]
    fn into_ptr(self) -> *mut Self {
        Box::leak(Box::new(self))
    }
}
