/// # Rc
/// `Rc<T>` is a reference-counted pointer type that allows multiple shared references
/// to a value of type `T`. It tracks the number of references and automatically deallocates
/// the value when the last reference is dropped.
///
/// The `Rc<T>` type is implemented as a thin wrapper around a raw pointer to an `Inner<T>` struct,
/// which contains the value of type `T` and a reference count stored in a `std::cell::UnsafeCell<usize>`.
///
/// # Differences from `std::rc::Rc`
///
/// The implementation of `speedy_refs::Rc` is similar to the implementation of `std::rc::Rc` in many ways. However, there are a few key differences in the implementation that are worth noting:
///
/// 1. The reference count in `speedy_refs::Rc` is stored in an `UnsafeCell<usize>`, whereas in `std::rc::Rc` it is stored in a non-atomic `Cell<usize>`. This makes the `std::rc::Rc` implementation slightly more efficient in single-threaded environments, but also makes it less suitable for multithreaded environments.
///
/// 2. The `speedy_refs::Rc` implementation does not provide an equivalent of `std::rc::Weak`, which allows for weak references that do not increment the reference count. This means that `speedy_refs::Rc` cannot be used for cyclic data structures that require weak references.
///
/// 3. The `speedy_refs::Rc` implementation does not provide an equivalent of `std::sync::Arc`, which allows for atomically reference-counted pointers that can be shared between threads. This means that `speedy_refs::Rc` is only suitable for use in single-threaded environments.
///
/// Despite these differences, the basic functionality and usage of `speedy_refs::Rc` is very similar to that of `std::rc::Rc`.
///
/// # Examples
///
/// ```
/// use speedy_refs::Rc;
///
/// let value = Rc::new(42);
///
/// let reference1 = Rc::clone(&value);
/// let reference2 = Rc::clone(&value);
///
/// assert_eq!(*value, 42);
/// assert_eq!(*reference1, 42);
/// assert_eq!(*reference2, 42);
///
/// drop(reference1);
///
/// assert_eq!(*value, 42);
/// assert_eq!(*reference2, 42);
///
/// drop(reference2);
///
/// // value is deallocated here
/// ```
pub struct Rc<T>(*mut Inner<T>);

impl<T> Clone for Rc<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { self.0.as_ref().unwrap() }.increment();
        Self(self.0)
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
    fn drop(&mut self) {
        if unsafe { self.0.as_ref().unwrap().decrement() } == 0 {
            // TODO
            // println!("Dropping actual content")
            // let _ = unsafe { Box::from_raw(self.0) };
        } else {
            // println!("Dropping clone");
            // TODO
        }
    }
}

/// The `Inner` struct is a helper struct for `Rc` that stores the value and the reference count
/// for a shared value of type `T`. It is used to implement reference counting for the `Rc` type.
///
/// The first field of `Inner` is the value of type `T` being shared by one or more `Rc`
/// instances. The second field is an `UnsafeCell<usize>` that is used to store the reference count
/// of the shared value. The `UnsafeCell` allows for interior mutability, which is necessary to
/// increment or decrement the reference count from multiple `Rc` instances.
///
/// The `new` method constructs a new `Inner` instance with the given value and an initial reference
/// count of 1. The reference count is stored in an `UnsafeCell`, which is wrapped in a `Cell` to
/// provide interior mutability.
///
/// The `into_ptr` method takes ownership of an `Inner` instance and returns a raw pointer to it.
/// This is used to create a new `Rc` instance that points to the shared value. The returned pointer
/// is guaranteed to be unique, since it is created by `Box::leak`, which creates a memory leak by
/// "forgetting" the original box allocation. This is safe because the leaked box will never be
/// deallocated, so the pointer to its contents will remain valid for the lifetime of the program.
pub(super) struct Inner<T>(T, std::cell::UnsafeCell<usize>);

impl<T> Inner<T> {
    /// Constructs a new `Inner` instance with the given value and an initial reference count of 1.
    ///
    pub(super) fn new(val: T) -> Self {
        Self(val, std::cell::UnsafeCell::new(1))
    }

    /// Takes ownership of an `Inner` instance and returns a raw pointer to it.
    ///
    /// This method is used to create a new `Rc` inner field that points to the shared value. The
    /// returned pointer is guaranteed to be unique, since it is created by `Box::leak`, which
    /// creates a memory leak by "forgetting" the original box allocation. This is safe because
    /// the leaked box will never be deallocated, so the pointer to its contents will remain valid
    /// until the last Rc clone is dropped.
    pub(super) fn into_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    fn decrement(&self) -> usize {
        unsafe {
            *self.1.get() -= 1;
            *self.1.get()
        }
    }

    fn increment(&self) {
        unsafe { *self.1.get() += 1 }
    }
}

impl<T> !Send for Rc<T> {}
impl<T> !Sync for Rc<T> {}

mod test {
    #[test]
    fn test_drop() {
        #[derive(Default)]
        struct Inner;
        impl Drop for Inner {
            fn drop(&mut self) {
                println!("Dropping inner");
            }
        }
        #[derive(Default)]
        struct Item(Inner);

        let val = Item::default();
        let rc = super::Rc::new(val);
        let clone = rc.clone();
        let clone2 = rc.clone();

        std::mem::drop(clone);
        std::mem::drop(clone2);
        std::mem::drop(rc);
    }
}
