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