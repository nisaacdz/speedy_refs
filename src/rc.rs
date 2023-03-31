pub struct Rc<T> {
    inner: std::ptr::NonNull<Inner<T>>,
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let mut inner = self.inner;
            inner.as_mut().increment_count();
            Self { inner }
        }
    }
}

impl<T> Rc<T> {
    pub fn new(data: T) -> Self {
        let res = Inner::new(data).into_non_null_ptr();
        Self { inner: res }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().value() }
    }
}

impl<T> AsRef<T> for Rc<T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        if unsafe { self.inner.as_mut().decrement_count() } == 0 {
            unsafe { std::mem::drop(Box::from_raw(self.inner.as_mut().pointer())) }
        }
    }
}

struct Inner<T> {
    val: T,
    count: usize,
}

impl<T> Inner<T> {
    pub(super) fn new(val: T) -> Self {
        Self { val, count: 1 }
    }

    #[inline(always)]
    fn into_non_null_ptr(self) -> std::ptr::NonNull<Self> {
        std::ptr::NonNull::new(Box::into_raw(Box::new(self))).unwrap()
    }

    #[inline]
    fn value(&self) -> &T {
        &self.val
    }

    #[inline(always)]
    fn pointer(&mut self) -> *mut T {
        (&mut self.val) as *mut T
    }

    #[inline(always)]
    fn increment_count(&mut self) {
        self.count += 1;
    }

    /// Decreases reference count by one and returns the new count
    #[inline(always)]
    fn decrement_count(&mut self) -> usize {
        self.count -= 1;
        self.count
    }
}
