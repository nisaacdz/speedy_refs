#[allow(dead_code)]
pub struct AtomicPtr<T> {
    ptr: std::sync::atomic::AtomicPtr<T>,
}

impl<T> std::fmt::Pointer for AtomicPtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self, f)
    }
}

#[allow(unused)]
impl<T> AtomicPtr<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: std::sync::atomic::AtomicPtr::new(Box::leak(Box::new(value)) as *mut T),
        }
    }

    pub fn load_mut<R, F: Fn(&mut T) -> R>(&self, f: F, ordering: std::sync::atomic::Ordering) -> R {
        f(self.as_mut(ordering))
    }

    pub fn load_ref<R, F: Fn(&T) -> R>(&self, f: F, ordering: std::sync::atomic::Ordering) -> R {
        f(self.as_ref(ordering))
    }

    pub fn as_mut(&self, ordering: std::sync::atomic::Ordering) -> &mut T {
        todo!()
    }

    pub fn as_ref(&self, ordering: std::sync::atomic::Ordering) -> &T {
        todo!()
    }
}