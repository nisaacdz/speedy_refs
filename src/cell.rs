// # speedy_refs::HeapCell
///
/// `HeapCell` is a container type that allows mutation of its contents even when it is
/// externally immutable by storing the type it points to on the heap and keeping a mutable pointer to it.
///
/// Functions like `NonNull` + `UnsafeCell`
///
/// # Note
/// - It moves the data onto the heap and stores a pointer to it.
/// - It never drops or deallocates the data it wraps until `drop()` and `dealloc()` methods are explicitly called or `drop_n_dealloc()` is called.
///
/// # Uses
/// `HeapCell` can be accessed mutably through the `as_mut` method and immutably through the
/// `as_ref` method without requiring the container to be mutable.
///
/// # Examples
///
/// ```
/// use speedy_refs::HeapCell;
///
/// let cell = HeapCell::new(42);
/// assert_eq!(unsafe { *cell.as_ref() }, 42);
///
/// unsafe {
///     *cell.as_mut() = 7;
///     assert_eq!(*cell.as_ref(), 7);
///
///     let val = cell.take();
///     assert_eq!(val, 7);
///
///     cell.replace(42);
///     assert_eq!(*cell.as_ref(), 42);
///
///     cell.drop_n_dealloc();
/// }
/// ```

pub struct HeapCell<T> {
    inner: *mut T,
}

impl<T> Clone for HeapCell<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> HeapCell<T> {
    /// Creates a new `HeapCell` containing the given value.
    ///
    /// # Safety
    ///
    /// This function takes ownership of the given value and stores it behind a
    /// raw pointer. It is the responsibility of the caller to ensure that the
    /// value passed in is valid and the HeapCell is not used after the value has been
    /// dropped or moved.
    ///
    /// # Panics
    ///
    /// This function will panic if memory allocation fails.
    pub fn new(val: T) -> Self {
        Self {
            inner: Box::leak(Box::new(val)) as *mut T,
        }
    }

    /// Creates a mutable reference to T from an immutable reference to self.
    ///
    /// # Returns
    /// * **&mut T** - A mutable reference to the `self.inner` value
    ///
    /// # Safety
    /// This function returns a mutable reference to T by dereferencing a raw pointer.
    /// It is the responsibility of the caller to
    /// ensure that this method is not invoked while there is an outstanding
    /// **mutable** or **immutable** reference this same T, as that would lead to data races.
    ///
    pub unsafe fn as_mut(&self) -> &mut T {
        &mut *self.inner
    }

    /// Creates an immutable reference to self from an immutable reference to self.
    ///
    /// # Returns
    /// * &T - An immutable reference to T, the `self.inner` value
    ///
    /// # Safety
    /// This function returns an immutable reference to T by dereferencing a raw pointer.
    /// It is the responsibility of the caller to
    /// ensure that this method is not invoked while there is an outstanding
    /// **mutable** reference this same T, as that would lead to data races.
    pub unsafe fn as_ref(&self) -> &T {
        &*self.inner
    }

    /// Takes ownership of the value stored in the `HeapCell`.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it assumes ownership of the value
    /// behind the raw pointer held by the `HeapCell`. The caller must ensure that it is
    /// safe to take ownership of the value.
    ///
    /// # Returns
    ///
    /// This function returns the owned value of type `T` that was stored in the `HeapCell`.
    ///
    /// # Note
    ///
    /// The caller must ensure that the `HeapCell` is not used after calling `take`,
    /// as the `HeapCell` will be left in an invalid state.
    ///
    pub unsafe fn take(&self) -> T {
        // It is safe to call `from_raw` here, as the `self.inner` was originally created
        // using `Box::into_raw`.
        std::ptr::read(self.inner)
    }

    /// Drops the content and deallocates its memory.
    ///
    /// This function first calls drop on T and then deallocates the memory associated with it
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `HeapCell` is not used after calling `deallocate` as
    /// `self.inner` will then point to an invalid memory.
    pub unsafe fn drop_n_dealloc(&self) {
        let ptr = self.inner;
        std::ptr::drop_in_place(ptr);
        std::alloc::dealloc(ptr as *mut u8, std::alloc::Layout::new::<T>());
    }
    /// Creates a new cell wrapping a clone of the inner value.
    ///
    /// # Returns
    /// The new cloned value
    pub fn clone_inner(&self) -> HeapCell<T>
    where
        T: Clone,
    {
        let clone = unsafe { self.inner.as_ref().unwrap() }.clone();
        HeapCell::new(clone)
    }

    /// Replaces the wraped value with `val` and returns the original one
    ///
    /// # Safety
    /// It is up to the caller to make sure that this method is not called while T is borrowed
    pub unsafe fn replace(&self, mut val: T) -> T {
        std::mem::swap(unsafe { self.as_mut() }, &mut val);
        val
    }

    // Calls drop on T, if it implements Drop
    #[inline]
    pub unsafe fn drop(&self) {
        std::ptr::drop_in_place(self.inner);
    }

    // Deallocates the momory associated with T
    #[inline]
    pub unsafe fn dealloc(&self) {
        std::alloc::dealloc(self.inner as *mut u8, std::alloc::Layout::new::<T>());
    }
}

/// # BorrowFlag
/// For `immutably` and `safely` tracking the reads and writes to an owned value.
///
/// Rust rules allow a number of reads or a single write at a time.
///
/// # Fields
/// * `inner: std::cell::UnsafeCell<isize>`
///
/// # Note
/// BorrowFlag is meant to be added as a field in your struct for added borrow checker functionalities since it
/// doesn't store the actual value described
///
#[repr(transparent)]
pub struct BorrowFlag {
    inner: std::cell::UnsafeCell<isize>,
}

impl BorrowFlag {
    /// Initiallizes a new BorrowFlag with 0 current reads and no current writer
    pub fn new() -> Self {
        Self {
            inner: std::cell::UnsafeCell::new(0),
        }
    }

    /// Checks if the described value can be borrowed immutably
    ///
    /// # Note
    /// The value can only be borrowed immutably if there is currently no mutable references to it
    ///
    /// # Returns
    /// * `true` - If immutable borrow is possible
    /// * `false` - If immutable borrow is impossible
    pub fn can_borrow(&self) -> bool {
        unsafe { *self.inner.get() >= 0 }
    }

    /// Checks if the described value can be borrowed mutably
    ///
    /// # Note
    /// The value can only be borrowed mutably if there is currently no mutable or immutable references to it.
    ///
    /// # Returns
    /// * `true` - If mutable borrow is possible
    /// * `false` - If mutable borrow is impossible
    pub fn can_borrow_mut(&self) -> bool {
        unsafe { *self.inner.get() == 0 }
    }

    /// Checks if the described value can be taken ownership of
    ///
    /// # Note
    /// The value can only be owned if there is currently no mutable or immutable references to it.
    ///
    /// # Returns
    /// * `true` - If taking ownership is possible
    /// * `false` - If taking ownership is impossible

    pub fn can_own(&self) -> bool {
        self.can_borrow_mut()
    }
    /// Marks a the start of a new read by increasing the count of the internal `readers`
    ///
    pub fn borrow(&self) {
        unsafe { std::ptr::write(self.inner.get(), *self.inner.get() + 1) }
    }
    /// Marks the end of an ongoing read by decrementing the count of the internal `readers`
    ///
    ///
    pub fn drop_borrow(&self) {
        unsafe { std::ptr::write(self.inner.get(), *self.inner.get() - 1) }
    }

    /// Marks the start of a write by setting the internal `write` field to true
    ///
    pub fn borrow_mut(&self) {
        unsafe { std::ptr::write(self.inner.get(), -1) }
    }
    /// Marks the end of a write session by setting the internal `write` field to false
    ///
    pub fn drop_borrow_mut(&self) {
        unsafe { std::ptr::write(self.inner.get(), 0) }
    }
}

/// An immutable borrow of RefCell
pub struct Ref<'a, T> {
    val: &'a mut Inner<T>,
}

/// Dropping a Ref object
impl<'a, T> Drop for Ref<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.val.flag -= 1;
    }
}

impl<'a, T> std::ops::Deref for Ref<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val.val
    }
}

impl<'a, T> AsRef<T> for Ref<'a, T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

pub struct RefMut<'a, T> {
    val: &'a mut Inner<T>,
}

impl<'a, T> RefMut<'a, T> {
    pub fn replace(&mut self, val: T) -> T {
        std::mem::replace(&mut self.val.val, val)
    }
}

impl<'a, T> std::ops::Deref for RefMut<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.val.val
    }
}

impl<'a, T> AsRef<T> for RefMut<'a, T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

impl<'a, T> std::ops::DerefMut for RefMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val.val
    }
}

impl<'a, T> AsMut<T> for RefMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        std::ops::DerefMut::deref_mut(self)
    }
}

impl<'a, T> Drop for RefMut<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.val.flag = 0;
    }
}

/// # RefCell
/// A RefCell is a mutable memory location with dynamically checked borrow rules.
///
///
/// # vs std::cell::RefCell
/// //TODO
///
/// The `RefCell` stores a value of type `T`, and allows mutable access through the `borrow_mut` method,
/// which returns a `RefMut<T>` type. Immutable access is granted through the `borrow` method, which
/// returns a `Ref<T>` type.
///
/// # Panics
/// If any of the borrow rules are violated at runtime
///
/// # Others
/// The `take` method can be used to extract the value from the RefCell and invalidate the borrow flag.
///
/// The `Clone` trait is implemented for `RefCell<T>` only if `T` also implements `Clone`.
///
/// # Examples
///
/// ```
/// use speedy_refs::RefCell;
/// let x = RefCell::new(42);
/// // borrowing immutably
/// let y = x.borrow();
/// assert_eq!(*y, 42);
/// // attempting to borrow mutably while already borrowed immutably
/// // will panic at runtime ///
/// // let z = x.borrow_mut();
/// // uncomment to see the panic ///
///
/// // borrowing mutably
/// std::mem::drop(y);
/// let mut z = x.borrow_mut();
/// *z += 1;
/// assert_eq!(*z, 43);
/// // attempting to borrow immutably while already borrowed mutably will panic at runtime
/// /// // let y = x.borrow();
/// // uncomment to see the panic
///
/// // extracting the value from the RefCell
/// std::mem::drop(z);
/// let val = x.take();
/// assert_eq!(val, 43);
/// ```

pub struct RefCell<T> {
    inner: std::cell::UnsafeCell<Inner<T>>,
}

impl<T> RefCell<T> {
    /// Creates a new `RefCell` containing the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use speedy_refs::RefCell;
    ///
    /// let cell = RefCell::new(42);
    /// ```
    pub fn new(val: T) -> Self {
        Self {
            inner: std::cell::UnsafeCell::new(Inner::new(val)),
        }
    }

    /// Borrows the value immutably. Panics if the value is currently borrowed mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// use speedy_refs::RefCell;
    ///
    /// let cell = RefCell::new(42);
    ///
    /// let reference = cell.borrow();
    ///
    /// assert_eq!(*reference, 42);
    /// ```
    pub fn borrow<'a>(&'a self) -> Ref<'a, T> {
        self.try_borrow()
            .expect("T cannot be borrowed immutably while T is borrowed mutably")
    }

    /// Tries to borrow the value immutably. Returns `None` if the value is currently borrowed mutably.
    ///
    /// # Examples
    ///
    /// ```
    /// use speedy_refs::RefCell;
    ///
    /// let cell = RefCell::new(42);
    ///
    /// let reference1 = cell.try_borrow().unwrap();
    /// assert_eq!(*reference1, 42);
    ///
    /// let reference2 = cell.try_borrow();
    /// assert!(reference2.is_none());
    /// ```
    pub fn try_borrow<'a>(&'a self) -> Option<Ref<'a, T>> {
        unsafe {
            if (*self.inner.get()).flag == 0 {
                (&mut *self.inner.get()).flag += 1;
                Some(Ref {
                    val: &mut *self.inner.get(),
                })
            } else {
                None
            }
        }
    }

    /// Borrows the value mutably. Panics if the value is currently borrowed (either mutably or immutably).
    ///
    /// # Examples
    ///
    /// ```
    /// use speedy_refs::RefCell;
    ///
    /// let cell = RefCell::new(42);
    ///
    /// *cell.borrow_mut() = 13;
    ///
    /// let reference = cell.borrow();
    ///
    /// assert_eq!(*reference, 13);
    /// ```
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, T> {
        self.try_borrow_mut()
            .expect("T cannot be borrowed mutably while T is borrowed mutably or immutably")
    }

    /// Tries to borrow the value mutably. Returns `None` if the value is currently borrowed (either mutably or immutably).
    ///
    /// # Examples
    ///
    /// ```
    /// use speedy_refs::RefCell;
    ///
    /// let cell = RefCell::new(42);
    ///
    /// let mut mut_reference1 = cell.try_borrow_mut().unwrap();
    /// *mut_reference1 = 13;
    ///
    /// let mut_reference2 = cell.try_borrow_mut();
    /// assert!(mut_reference2.is_none());
    /// ```
    pub fn try_borrow_mut<'a>(&'a self) -> Option<RefMut<'a, T>> {
        unsafe {
            if (*self.inner.get()).flag == 0 {
                (&mut *self.inner.get()).flag = -1;
                Some(RefMut {
                    val: &mut *self.inner.get(),
                })
            } else {
                None
            }
        }
    }

    pub fn take(mut self) -> T {
        if self.inner.get_mut().flag == 0 {
            let Inner { val, flag: _ } = self.inner.into_inner();
            val
        } else {
            panic!("T cannot be moved while T is borrowed mutably or immutably")
        }
    }
    /// Replaces the wrapped value with a new one computed from f, returning the old value, without deinitializing either one.
    ///
    /// # Panics
    ///Panics if the value is currently borrowed.
    pub fn replace(&self, val: T) -> T {
        self.borrow_mut().replace(val)
    }
}

impl<T: Clone> Clone for RefCell<T> {
    fn clone(&self) -> Self {
        unsafe { RefCell::new((*self.inner.get()).val.clone()) }
    }
}

struct Inner<T> {
    val: T,
    flag: isize,
}

impl<T> Inner<T> {
    fn new(val: T) -> Self {
        Self { val, flag: 0 }
    }
}

unsafe impl<T: Send> Send for RefCell<T> {}

/// # speedy_refs::RcCell
/// A reference-counted cell that allows for interior mutability.
///
/// This struct is essentially a zero-cost abstraction over using `std::rc::Rc<std::cell::RefCell<T>>`,
/// allowing for easier and more concise code. Multiple `RcCell` instances can share ownership of the
/// same value, and the value can be mutated even when there are shared references to it.
///
/// # Example
/// ```
/// use speedy_refs::RcCell;
/// let rc_cell = RcCell::new(42);
/// let shared_rc_cell = rc_cell.clone();
/// // Get a shared reference to the value inside the RcCell.
///
/// // Update the value inside the RcCell.
/// *rc_cell.borrow_mut() += 1;
///
/// let shared_ref = shared_rc_cell.borrow();
///
/// // The shared reference reflects the updated value.
/// assert_eq!(*shared_ref, 43);
/// ```

pub struct RcCell<T> {
    inner: std::rc::Rc<std::cell::RefCell<T>>,
}

impl<T> RcCell<T> {
    /// Creates a new `RcCell<T>` instance containing the provided value.
    pub fn new(value: T) -> RcCell<T> {
        Self {
            inner: std::rc::Rc::new(std::cell::RefCell::new(value)),
        }
    }
}

impl<T> Clone for RcCell<T> {
    /// Clones the `RcCell<T>` instance, creating a new instance that shares ownership of the same value.
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> std::ops::Deref for RcCell<T> {
    type Target = std::cell::RefCell<T>;
    /// Dereferences the `RcCell<T>` instance to the underlying `RefCell<T>`.
    fn deref(&self) -> &Self::Target {
        std::ops::Deref::deref(&self.inner)
    }
}

/// Freely share multiple mutable references within a single thread.
///
///
/// A smart pointer that provides shared ownership of its contained value `T` in a single-threaded environment.
///
/// This struct uses an `UnsafeCell` internally to allow for interior mutability, which means that the value can be mutated
/// even if there are multiple references to it. However, it is not safe for concurrent access from multiple threads.
/// 
/// # Examples
/// ```
/// use speedy_refs::SharedCell;
/// // Create a new SharedCell with the initial value 42.
/// let cell = SharedCell::new(42);
/// // Get a shared reference to the contained value.
/// let value_ref = unsafe { cell.as_ref() };
/// assert_eq!(*value_ref, 42);
/// // Get a mutable reference to the contained value.
/// let value_mut_ref = unsafe { cell.as_mut() };
/// *value_mut_ref = 10;
/// assert_eq!(*value_ref, 10);
/// ```
pub struct SharedCell<T> {
    value: std::cell::UnsafeCell<T>,
}

impl<T> SharedCell<T> {
    /// Creates a new `SharedCell` instance with the specified initial value.
    pub fn new(value: T) -> SharedCell<T> {
        Self {
            value: std::cell::UnsafeCell::new(value),
        }
    }

    /// Returns a mutable reference to the contained value.
    ///
    /// This method uses the `UnsafeCell` internally to allow for mutable access without violating Rust's borrowing rules.
    /// However, it is marked as unsafe because it allows for multiple mutable references to the same value, which can lead
    /// to data races if not used correctly.
    pub unsafe fn as_mut(&self) -> &mut T {
        &mut *self.value.get()
    }

    /// Returns a shared reference to the contained value.
    ///
    /// This method uses the `UnsafeCell` internally to allow for shared access without violating Rust's borrowing rules.
    /// However, it is marked as unsafe for the same reason as `as_mut`.
    pub unsafe fn as_ref(&self) -> &T {
        &*self.value.get()
    }
}

// We mark `SharedCell` as not `Sync` because it is not safe for concurrent access from multiple threads.
impl<T> !Sync for SharedCell<T> {}

// We mark `SharedCell` as `Send` if the contained type `T` is also `Send`.
unsafe impl<T: Send> Send for SharedCell<T> {}

/// # JavaCell
/// A smart pointer that provides shared ownership of its contained value `T` with interior mutability.
///
/// This struct is implemented using an `std::rc::Rc` and a `speedy_refs::SharedCell` to provide shared ownership and interior mutability,
/// respectively. It behaves similarly to a `Cell` in that its contents can be modified through a shared reference.
///
/// # Note
/// Does not provide runtime borrow checking. If you want runtime borrow checking use `RcCell` instead.
/// 
/// # Examples
///
/// ```
/// use speedy_refs::JavaCell;
///
/// let mut items: JavaCell<Vec<String>> = JavaCell::new(vec!["Hello".into(), ",".into(), " ".into(), "World".into(), "!".into()]);
///
/// let clone = items.clone();
///
/// // Modify the contents of the original JavaCell.
/// items.pop();
///
/// // Collect shared references to the remaining items.
/// let mut refs = items.iter().collect::<Vec<&String>>();
///
/// assert_eq!(clone.len(), 4);
/// ```
pub struct JavaCell<T> {
    value: std::rc::Rc<SharedCell<T>>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for JavaCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(unsafe { self.value.as_ref().as_ref() }, f)
    }
}

impl<T> JavaCell<T> {
    /// Creates a new `JavaCell` instance with the specified initial value.
    pub fn new(value: T) -> JavaCell<T> {
        Self {
            value: std::rc::Rc::new(SharedCell::new(value)),
        }
    }
}

impl<T> std::ops::Deref for JavaCell<T> {
    type Target = T;

    /// Returns a shared reference to the contained value.
    ///
    /// This method uses the `UnsafeCell` internally to allow for shared access without violating Rust's borrowing rules.
    /// However, it is marked as unsafe for the same reason as `SharedCell::as_ref`.
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref().as_ref() }
    }
}

impl<T> std::ops::DerefMut for JavaCell<T> {
    /// Returns a mutable reference to the contained value.
    ///
    /// This method uses the `UnsafeCell` internally to allow for mutable access without violating Rust's borrowing rules.
    /// However, it is marked as unsafe for the same reason as `SharedCell::as_mut`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_ref().as_mut() }
    }
}

impl<T> Clone for JavaCell<T> {
    /// Returns a new `JavaCell` instance with a shared reference to the same contained value.
    ///
    /// The contents of the `JavaCell` can be modified through any of its clones, but each clone still owns a shared reference
    /// to the same value. When all clones are dropped, the value will be dropped as well.
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
