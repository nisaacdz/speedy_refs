pub struct Reon<T> {
    inner: std::sync::Arc<T>,
}

impl<T> Clone for Reon<T> {
    fn clone(&self) -> Self {
        Self {
            inner: std::sync::Arc::clone(&self.inner),
        }
    }
}

impl<T> std::ops::Deref for Reon<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T> AsRef<T> for Reon<T> {
    fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<T> Reon<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: std::sync::Arc::new(data),
        }
    }
}

unsafe impl<T> Sync for Reon<T> {}
unsafe impl<T> Send for Reon<T> {}

pub struct Rajax<T> {
    inner: *const T,
}

impl<T> Clone for Rajax<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<T> Copy for Rajax<T> {

}

impl<T> std::ops::Deref for Rajax<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap() }
    }
}

impl<T> AsRef<T> for Rajax<T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

impl<T> Rajax<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: Box::into_raw(Box::new(value)),
        }
    }
}

unsafe impl<T> Send for Rajax<T> {}
unsafe impl<T> Sync for Rajax<T> {}
