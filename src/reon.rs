/// # Reon (Read Only)
/// A read-only smart pointer that points to a static heap data.
///
/// It is useful for:
/// * Types that need to be used immutably from different threads throughout the entire program's duration.
/// * Safely converting non-static values into static values.
///
/// # How it works
/// Reon leaks the type onto the heap and stores it. This allows it to have a static lifetime.
///
/// # Interior Mutability
/// Reon is not designed for interior mutability and should not be used with it. It does not employ any form of reference counting or mutual exclusion principles for accessing the data it points to. Therefore, it makes no data-race guarantees.
///
/// # Traits
/// * It implements `Copy` and `Clone` which both clone the pointer to the underlying data without cloning the data itself.
/// * It implements `Deref` and `AsRef` with both target types as T.
///
/// # Examples
///
/// ## Initialization
///
/// ```
/// use speedy_refs::Reon;
/// let x = Reon::new(42);
/// assert_eq!(*x, 42);
/// ```
///
/// ## Moving accross Threads
/// ```
/// use std::thread;
/// use speedy_refs::Reon;
///
/// fn main() {
///     let x = Reon::new(42);
///     let num_threads = 4;
///
///     let mut handles = Vec::with_capacity(num_threads);
///
///
///     for _ in 0..num_threads {
///         let x = x.clone();
///         let thread = thread::spawn(move || {
///             // Do some immutable stuff with x here
///             assert_eq!(42, *x);
///         });
///         handles.push(thread);
///     }
///
///     for thread in handles {
///         thread.join().unwrap();
///     }
///}

/// ```
#[derive(Copy)]
pub struct Reon<T: 'static> {
    inner: &'static T,
}

impl<T> Clone for Reon<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<T> std::ops::Deref for Reon<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<T> AsRef<T> for Reon<T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

impl<T> Reon<T> {
    /// Constructs a new `Reon<T>` from a given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use speedy_refs::Reon;
    /// let x = Reon::new(42);
    /// assert_eq!(*x, 42);
    /// ```
    pub fn new(value: T) -> Self {
        Self {
            inner: Box::leak(Box::new(value)),
        }
    }
}

unsafe impl<T> Send for Reon<T> {}
unsafe impl<T> Sync for Reon<T> {}
