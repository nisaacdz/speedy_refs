pub struct Arc<T> {
    inner: *mut Inner<T>,
}

impl<T> Clone for Arc<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            let inner = self.inner.clone();
            inner.as_mut().unwrap().increment_count();

            Self { inner }
        }
    }
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        let res = Inner::new(data).into_ptr();
        Self { inner: res }
    }
}

impl<T> std::ops::Deref for Arc<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().unwrap().value() }
    }
}

impl<T> AsRef<T> for Arc<T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_mut().unwrap() };

        let new_count = inner.decrement_count();

        if new_count == 1 {
            let _ = unsafe { Box::from_raw(self.inner) };
        }
    }
}

struct Inner<T> {
    ptr: T,
    count: std::sync::atomic::AtomicUsize,
}

impl<T> Inner<T> {
    fn new(data: T) -> Self {
        Self {
            ptr: data,
            count: std::sync::atomic::AtomicUsize::new(1),
        }
    }

    #[inline(always)]
    fn into_ptr(self) -> *mut Inner<T> {
        Box::leak(Box::new(self))
    }

    #[inline(always)]
    fn value(&self) -> &T {
        &self.ptr
    }

    #[inline(always)]
    fn increment_count(&mut self) {
        self.count
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }

    /// Decreases reference count by one and returns the old value
    #[inline(always)]
    fn decrement_count(&mut self) -> usize {
        self.count
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel)
    }
}

unsafe impl<T: Sync + Send> Sync for Arc<T> {}
unsafe impl<T: Sync + Send> Send for Arc<T> {}
