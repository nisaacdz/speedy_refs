pub struct Rc<T> {
    inner: super::HeapCell<Inner<T>>,
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.inner.as_mut().increment_count();
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}

impl<T> Rc<T> {
    pub fn new(data: T) -> Self {
        let res = super::HeapCell::new(Inner::new(data));
        Self { inner: res }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    #[inline]
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
        unsafe {
            if self.inner.as_mut().decrement_count() == 0 {
                self.inner.deallocate()
            }
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

    #[inline]
    fn value(&self) -> &T {
        &self.val
    }

    #[inline]
    fn increment_count(&mut self) {
        self.count += 1;
    }

    /// Decreases reference count by one and returns the new count
    #[inline]
    fn decrement_count(&mut self) -> usize {
        self.count -= 1;
        self.count
    }
}
