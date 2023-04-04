#[allow(dead_code)]
pub struct AtomicPtr<T> {
    ptr: *mut T,
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
            ptr: Box::into_raw(Box::new(value)),
        }
    }

    pub fn load_mut<R, F: Fn(&mut T) -> R>(&self, f: F, ordering: Ordering) -> R {
        f(self.as_mut(ordering))
    }

    pub fn load_ref<R, F: Fn(&T) -> R>(&self, f: F, ordering: Ordering) -> R {
        f(self.as_ref(ordering))
    }

    pub fn as_mut(&self, ordering: Ordering) -> &mut T {
        todo!()
    }

    pub fn as_ref(&self, ordering: Ordering) -> &T {
        todo!()
    }
}

pub enum Ordering {
    Release,
    Relax,
}
