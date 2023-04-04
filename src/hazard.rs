/// Use at your own risk!
///
/// This Reference provides shared mutable access to multiple threads
///  simultaneously without any locking or synchronization mechanism
/// It provides zero guarantee that

#[derive(Clone)]
pub struct HazardCell<T> {
    inner: Inner<T>,
}

impl<T> std::ops::Deref for HazardCell<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> std::ops::DerefMut for HazardCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> HazardCell<T> {
    #[inline]
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: Inner::new(value),
        }
    }

    pub fn as_mut(&self) -> &mut T {
        self.inner.as_mut()
    }

    pub fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

#[derive(Clone)]
struct Inner<T> {
    val: std::sync::Arc<std::cell::UnsafeCell<T>>,
}

impl<T> Inner<T> {
    fn new(val: T) -> Self {
        Self {
            val: std::sync::Arc::new(std::cell::UnsafeCell::new(val)),
        }
    }
    fn as_mut(&self) -> &mut T {
        unsafe { &mut *self.val.get() }
    }

    fn as_ref(&self) -> &T {
        unsafe { &*self.val.get() }
    }
}

unsafe impl<T> Sync for HazardCell<T> {}
unsafe impl<T> Send for HazardCell<T> {}
