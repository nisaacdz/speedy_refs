/// # speedy_refs::Rc
/// `Rc<T>` is a reference-counted pointer type that allows multiple shared references
/// to a value of type `T`. It tracks the number of references and automatically deallocates
/// the value when the last reference is dropped.
///
/// # Implementation
/// The `Rc<T>` type is implemented as a thin wrapper around a raw pointer to an `Inner<T>` struct,
/// which contains the value of type `T` and a reference count stored in a `std::cell::UnsafeCell<usize>`.
/// - The value passed to the new() is used together with a count to form an `Inner` type. let's call it `inner`.
/// - `inner` is moved to the heap
/// - A pointer to the heap memory of `inner` is kept by the `Rc` struct
/// - When the last Rc is dropped, `inner` is deallocated
/// 
/// # Weak References
///
/// This `Rc<T>` implementation does not provide a way to distinguish between strong and weak references.
/// Forming reference cycles with `Rc<T>` instances can lead to memory leaks, even after all strong references have been dropped.
/// To avoid memory leaks caused by reference cycles, we recommend that you use `std::rc::Rc` when the use case it likely
/// to form reference cycles.
///
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

/// Cloning An `Rc<T>` only creates a new pointer to the same content.
///
/// For this reason T has no Clone bound.
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        // Self.0 remains valid until the last reference is dropped.
        // For this reason it is safe to unwrap the `Option`
        unsafe { self.0.as_ref().unwrap() }.increment();
        Self(self.0)
    }
}

impl<T> Rc<T> {
    /// Creates a new `speedy_refs::Rc` instance and returns it.
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
            // println!("Dropping actual content");
            let _ = unsafe { Box::from_raw(self.0) };
            // unsafe { self.0.drop_in_place() }
            /*
            unsafe {
                // std::ptr::drop_in_place(self.0);
                std::alloc::dealloc(
                    self.0.cast(),
                    std::alloc::Layout::for_value(self.0.as_ref().unwrap()),
                )
            }
            */
        } else {
            // println!("Dropping clone");
            // TODO
        }
    }
}

/// # Inner
/// A helper struct for `Rc` that stores the value and the reference count
/// for a shared value of type `T`. It is used to implement reference counting for the `Rc` type.
///
/// The first field of `Inner` is the value of type `T` being shared by one or more `Rc`
/// instances. The second field is an `UnsafeCell<usize>` that is used to store the reference count
/// of the shared value. The `UnsafeCell` allows for interior mutability, which is necessary to
/// increment or decrement the reference count from immutable context.

struct Inner<T>(T, std::cell::UnsafeCell<usize>);

impl<T> Inner<T> {
    /// Constructs a new `Inner` instance with the given value and an initial reference count of 1.
    ///
    pub(super) fn new(val: T) -> Self {
        Self(val, std::cell::UnsafeCell::new(1))
    }

    /// Takes ownership of an `Inner` instance and returns a raw pointer to it.

    pub(super) fn into_ptr(self) -> *mut Self {
        Box::into_raw(Box::new(self))
    }

    /// Immutably decrement the count of the clones of the `Rc`

    fn decrement(&self) -> usize {
        unsafe {
            *self.1.get() -= 1;
            *self.1.get()
        }
    }

    // Immutably increment the count of the clones of the `Rc`

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

        impl Drop for Item {
            fn drop(&mut self) {
                println!("Dropping item");
            }
        }

        let val = Item::default();
        let rc = super::Rc::new(val);
        let _v = rc.clone();
        let _v = rc.clone();
    }

    #[test]
    fn test_1() {
        assert_eq!(1, 1)
    }
}
