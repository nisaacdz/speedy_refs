/// # HeapCell
///
/// `HeapCell` is a container type that allows mutation of its contents even when it is
/// externally immutable by storing the type it points to on the heap and keeping a mutable pointer to it.
/// 
/// # Like UnsafeCell
/// It is similar to `UnsafeCell` in terms of functionality but provides the following differences:
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub unsafe fn replace(&self, mut val: T) -> T {
        std::mem::swap(unsafe { self.as_mut() }, &mut val);
        val
    }

    // Calls drop on T, if it implements Drop
    pub unsafe fn drop(&self) {
        std::ptr::drop_in_place(self.inner);
    }

    // Deallocates the momory associated with T
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
    #[inline]
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
    #[inline]
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
    #[inline]
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

    #[inline]
    pub fn can_own(&self) -> bool {
        self.can_borrow_mut()
    }
    /// Marks a the start of a new read by increasing the count of the internal `readers`
    ///
    #[inline]
    pub fn borrow(&self) {
        unsafe {
            std::ptr::write(self.inner.get(), *self.inner.get() + 1)
        }
    }
    /// Marks the end of an ongoing read by decrementing the count of the internal `readers`
    ///
    #[inline]
    pub fn drop_borrow(&self) {
        unsafe {
            std::ptr::write(self.inner.get(), *self.inner.get() - 1)
        }
    }

    /// Marks the start of a write by setting the internal `write` field to true
    #[inline]
    pub fn borrow_mut(&self) {
        unsafe {
            std::ptr::write(self.inner.get(), -1)
        }
    }
    /// Marks the end of a write session by setting the internal `write` field to false
    #[inline]
    pub fn drop_borrow_mut(&self) {
        unsafe {
            std::ptr::write(self.inner.get(), 0)
        }
    }
}

/// An immutable borrow of RefCell
pub struct Ref<'a, T> {
    cell: &'a RefCell<T>,
}

/// Dropping a Ref object
impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        self.cell.flag.drop_borrow()
    }
}

impl<'a, T> std::ops::Deref for Ref<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        // Since self is the only mutable reference to the RefCell,
        // we can create references without minding to count it in the borrow flag
        unsafe { self.cell.inner.as_ref() }
    }
}

impl<'a, T> AsRef<T> for Ref<'a, T> {
    fn as_ref(&self) -> &T {
        std::ops::Deref::deref(self)
    }
}

pub struct RefMut<'a, T> {
    cell: &'a RefCell<T>,
}

impl<'a, T> RefMut<'a, T> {
    #[inline]
    pub fn new(val: &'a RefCell<T>) -> Self {
        Self { cell: val }
    }
    #[inline]
    pub fn replace(&mut self, val: T) -> T {
        unsafe { self.cell.inner.replace(val) }
    }
}

impl<'a, T> Drop for RefMut<'a, T> {
    #[inline]
    fn drop(&mut self) {
        self.cell.flag.drop_borrow_mut()
    }
}

impl<'a, T> std::ops::Deref for RefMut<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        // Since self is the only mutable reference to the RefCell,
        // we can create references without minding to count it in the borrow flag
        unsafe { self.cell.inner.as_ref() }
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
        unsafe { self.cell.inner.as_mut() }
    }
}

impl<'a, T> AsMut<T> for RefMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        std::ops::DerefMut::deref_mut(self)
    }
}

/// # RefCell
/// A RefCell is a mutable memory location with dynamically checked borrow rules.
///
/// It is similar to `HeapCell<T>`, but provides the ability to perform mutable borrows and immutable borrows safely
/// at runtime by runtime enforcement of the borrow-checker rules
///
/// The `RefCell` stores a value of type `T`, and allows mutable access through the `borrow_mut` method,
/// which returns a `RefMut<T>` type. Immutable access is granted through the `borrow` method, which
/// returns a `Ref<T>` type.
///
/// # BorrowFlag
/// Borrow rules are enforced at runtime, with a `BorrowFlag` type that keeps track of the number of active
/// borrows. If an attempt is made to borrow a value mutably while it is already borrowed (either `mutably` or
/// `immutably`), then a panic will occur. If an attempt is made to borrow a value `immutably` while it is already
/// `mutably` borrowed, then a panic will also occur.
///
/// # Panics
/// If the borrow rules are violated at runtime
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
    inner: HeapCell<T>,
    flag: BorrowFlag,
}

impl<T> RefCell<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: HeapCell::new(val),
            flag: BorrowFlag::new(),
        }
    }

    pub fn from_parts(val: HeapCell<T>, flag: BorrowFlag) -> Self {
        Self { inner: val, flag }
    }

    pub fn borrow<'a>(&'a self) -> Ref<'a, T> {
        if self.flag.can_borrow() {
            self.flag.borrow();
            Ref { cell: self }
        } else {
            panic!("T cannot be borrowed immutably while T is borrowed mutably")
        }
    }

    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, T> {
        if self.flag.can_borrow_mut() {
            self.flag.borrow_mut();
            RefMut { cell: self }
        } else {
            panic!("T cannot be borrowed mutably while T is borrowed mutably or immutably")
        }
    }

    pub fn take(self) -> T {
        if self.flag.can_own() {
            unsafe { self.inner.take() }
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

impl<T> Drop for RefCell<T> {
    fn drop(&mut self) {
        unsafe { self.inner.drop_n_dealloc() }
    }
}

impl<T: Clone> Clone for RefCell<T> {
    fn clone(&self) -> Self {
        let clone = self.inner.clone_inner();
        RefCell::from_parts(clone, BorrowFlag::new())
    }
}
