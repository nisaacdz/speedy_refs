pub struct Arc<T> {
    inner: std::ptr::NonNull<Inner<T>>,
}

impl<T> Clone for Arc<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            let mut inner = self.inner.clone();
            inner.as_mut().increment_count();

            Self { inner }
        }
    }
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        let res = Inner::new(data).into_non_null_ptr();
        Self { inner: res }
    }
}

impl<T> std::ops::Deref for Arc<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref().value() }
    }
}

impl<T> AsRef<T> for Arc<T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_mut() };

        let new_count = inner.decrement_count();

        if new_count == 1 {
            unsafe { self.inner.as_mut().deallocate() }
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
    fn into_non_null_ptr(self) -> std::ptr::NonNull<Self> {
        std::ptr::NonNull::new(Box::leak(Box::new(self)) as *mut Inner<T>).unwrap()
    }

    #[inline(always)]
    fn value(&self) -> &T {
        &self.ptr
    }

    #[inline(always)]
    fn pointer(&mut self) -> *mut T {
        &mut self.ptr as *mut T
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

    #[inline]
    unsafe fn deallocate(&mut self) {
        let ptr = self.pointer();
        std::ptr::drop_in_place(ptr);
        std::alloc::dealloc(ptr as *mut u8, std::alloc::Layout::new::<Self>());
    }
}

unsafe impl<T: Sync + Send> Sync for Arc<T> {}
unsafe impl<T: Sync + Send> Send for Arc<T> {}
