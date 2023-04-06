/// # Cell<T>
///  A `Cell` is a type that provides interior mutability, allowing for a value to be mutated
/// even when it is only immutably borrowed. It works by storing the value behind a raw pointer,
/// and providing methods for accessing and modifying it.
///
/// `Cell` provides the following methods:
///
/// - `new(val: T) -> Cell<T>`: creates a new Cell containing the given value.
/// - `as_mut(&self) -> &mut T`: creates a mutable reference to the Cell's inner value
/// from an immutable reference to self.
/// - `as_ref(&self) -> &T`: creates an immutable reference to the Cell's inner value
/// from an immutable reference to self.
/// - `take(&self) -> T`: takes ownership of the value stored in the Cell.
/// - `deallocate(&self)`: deallocates the memory of self.inner.
/// - `clone_inner(&self) -> Cell<T>`: creates a new Cell wrapping a clone of the inner value.
/// - `replace(&self, val: T) -> T`: replaces the wrapped value with val and returns the original one.
///
///
/// # Safety
///
///
/// All methods that allow for mutable access to the Cell's inner value are marked as `unsafe`,
/// as they rely on dereferencing a raw pointer, and the caller must ensure that the Cell is not used
/// while there are outstanding references to the same value. The take method is also marked as unsafe,
/// as it assumes ownership of the value behind the raw pointer. The deallocate method is marked as
/// unsafe because it assumes that the Cell is no longer in use and deallocates the memory associated with it.
///
///
/// # Examples
///
///
/// ```
/// use speedy_refs::Cell;
/// let item = Cell::new(3);
/// let val = *unsafe { item.as_ref() };
/// assert_eq!(3, val);
///
/// unsafe {
///     *item.as_mut() += 1;
/// }
///
/// let val = *unsafe { item.as_ref() };
/// assert_eq!(4, val);
/// ```
///  
/// # Deallocation
///
/// A cell does not automatically deallocate the memory of the value it wraps when it goes out of scope.
/// It is therefore the responsibily of the user to deallocate the contents of the cell after use
/// by calling the `deallocate()` method.
///
/// Also, a cell may not be used after deallocation because it will lead to undefined behaviour
///
/// ```
/// use speedy_refs::Cell;
///
/// let item = Cell::new(String::from("Hello, World!"));
/// assert_eq!("Hello, World!", unsafe { item.as_ref() });
///
/// unsafe{
///     item.deallocate();
/// }
/// ```
#[repr(transparent)]
#[derive(Copy)]
pub struct Cell<T> {
    inner: *mut T,
}

/// Cloning a `speedy_refs::Cell<T>` only clones the pointer to the underlining data
/// Since this `Cell<T>` is so cheap to clone, it also implements `Copy`
impl<T> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Cell<T> {
    /// Creates a new `Cell` containing the given value.
    ///
    /// # Note
    ///
    /// This moves the value onto the heap before keeping a raw pointer to it
    #[inline]
    pub fn new(val: T) -> Self {
        Self {
            /// Using `Box::leak` is more staightforward here than `Box::into_raw`
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
    /// # Example
    /// ```
    /// use speedy_refs::Cell;
    ///
    /// let item = Cell::new(2);
    ///
    /// unsafe {
    ///     *item.as_mut() += 1;
    ///
    ///     assert_eq!(3, *item.as_ref());
    /// }
    /// ```
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
    ///
    /// # Example
    ///
    /// ```
    /// use speedy_refs::Cell;
    ///
    /// let item = Cell::new(4usize);
    ///
    /// let val = * unsafe {
    ///     item.as_ref()
    /// };
    /// assert_eq!(4, val);
    /// ```
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        &*self.inner
    }

    /// Takes ownership of the value stored in the `Cell`.
    ///
    /// # Safety
    ///
    /// This function is marked as unsafe because it assumes ownership of the value
    /// behind the raw pointer held by the `Cell`. The caller must ensure that it is
    /// safe to take ownership of the value.
    ///
    /// # Returns
    ///
    /// This function returns the owned value of type `T` that was stored in the `Cell`.
    ///
    /// # Note
    ///
    /// The caller must ensure that the `Cell` is not used after calling `take`,
    /// as the `Cell` will be left in an invalid state.
    ///
    pub unsafe fn take(&self) -> T {
        // It is safe to call `from_raw` here, as the `self.inner` was originally created
        // using `Box::into_raw`.
        std::ptr::read(self.inner)
    }

    /// Deallocates the value wrapped by this cell
    ///
    /// This function first calls drop on T and then deallocates the memory associated with it
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `Cell` is not used after calling `deallocate` as
    /// `self.inner` will then point to an invalid memory.
    ///
    /// # Example
    ///
    /// ```
    /// use speedy_refs::Cell;
    ///
    /// let item = Cell::new(String::from("Hello, World!"));
    /// // Do some stuff with item
    /// unsafe {
    ///     item.deallocate();
    /// } // Deallocate the String stored in item
    ///
    /// // Do some other stuff
    /// ```
    ///
    pub unsafe fn deallocate(&self) {
        let ptr = self.inner;
        std::ptr::drop_in_place(ptr);
        std::alloc::dealloc(ptr as *mut u8, std::alloc::Layout::new::<T>());
    }
    /// Creates a new cell wrapping a clone of the inner value.
    ///
    /// # Returns
    /// The new Cell containing the cloned value
    ///
    /// # Example
    ///
    /// ```
    /// use speedy_refs::Cell;
    ///
    /// let stmt = "Hello, World";
    /// let mycell = Cell::new(String::from(stmt));
    ///
    ///unsafe {
    ///     let newcell = mycell.clone_inner();
    ///     mycell.as_mut().push('!');
    ///     assert_eq!(&mycell.as_ref()[..stmt.len()], &newcell.as_ref()[..]);
    ///     newcell.deallocate();
    ///     mycell.deallocate();
    /// }
    /// ```
    ///
    /// # T
    /// T must implement `Clone`
    #[inline]
    pub fn clone_inner(&self) -> Cell<T>
    where
        T: Clone,
    {
        let clone = unsafe { self.inner.as_ref().unwrap() }.clone();
        Cell::new(clone)
    }

    /// Replaces the wraped value with `val` and returns the original value
    ///
    /// # Safety
    /// It is up to the caller to make sure that this method is not called while T is borrowed
    #[inline]
    pub unsafe fn replace(&self, mut val: T) -> T {
        std::mem::swap(unsafe { self.as_mut() }, &mut val);
        val
    }
}

/// # BorrowFlag
/// For `immutably` and `safely` tracking the reads and writes to an owned value.
///
/// Rust rules allow a number of reads or a single write at a time.
///
/// # Fields
/// * `inner: Cell<isize>`
///
/// if `inner` is above zero, it represents the number of active immutable borrows
/// if `inner` is below zero, it means there is an active mutable borrow
///
/// # Note
/// BorrowFlag is meant to be added as a field in your struct together with the Cell type for added borrow checker functionalities since it
/// doesn't store the actual value described
///
/// # Basic Usage
///
/// ```
/// use speedy_refs::{Cell, BorrowFlag};
///
/// struct MyRef {
///     val: Cell<String>,
///     flag: BorrowFlag,
/// }
///
/// impl MyRef {
///     fn borrow(&self) -> Option<&String> {
///         if self.flag.can_borrow() {
///             unsafe {
///                 self.flag.borrow();
///                 Some(self.val.as_ref())
///             }
///         } else {
///             None
///         }
///     }
///
///     fn borrow_mut(&self) -> Option<&mut String> {
///         if self.flag.can_borrow_mut() {
///             unsafe {
///                 self.flag.borrow_mut();
///                 Some(self.val.as_mut())
///             }
///         } else {
///             None
///         }
///     }
/// }
/// ```
pub struct BorrowFlag {
    inner: Cell<isize>,
}

impl BorrowFlag {
    /// Initiallizes a new BorrowFlag with no borrows
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Cell::new(0),
        }
    }

    /// Initailizes a new BorrowFlag with the given value
    #[inline]
    pub fn new_from(val: isize) -> Self {
        Self {
            inner: Cell::new(val),
        }
    }

    /// Checks if the described value can be borrowed immutably
    ///
    /// # Note
    /// The value can only be borrowed immutably if it currently has no mutable borrows
    ///
    /// # Returns
    /// * `true` -> If immutable borrow is possible
    /// * `false` -> If immutable borrow is impossible
    #[inline]
    pub fn can_borrow(&self) -> bool {
        unsafe { *self.inner.as_ref() >= 0 }
    }

    /// Checks if the described value can be borrowed mutably
    ///
    /// # Note
    /// The value can only be borrowed mutably if it has no active mutable or immutable borrows
    ///
    /// # Returns
    /// * `true` - If mutable borrow is possible
    /// * `false` - If mutable borrow is impossible
    #[inline]
    pub fn can_borrow_mut(&self) -> bool {
        unsafe { *self.inner.as_ref() == 0 }
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
    /// Marks a the start of a new `immutable borrow` session or
    ///
    /// Informs the `BorrowFlag` that a new immutable borrow session has started
    ///
    /// # Safety
    /// This function is marked unsafe because the caller must guarrantee the following:
    ///
    /// * The data can currently be borrowed immutably or that the call to `can_borrow()` returns true
    /// * The data is actually borrowed immutably corresponding with a call to `borrow()`
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use speedy_refs::{Cell, BorrowFlag};
    /// struct Data(usize);
    ///
    /// impl Data {
    ///     fn new(val: usize) -> Self {
    ///         Self(val)
    ///     }
    ///     fn value(&self) -> usize {
    ///         self.0
    ///     }
    /// }
    ///
    /// let item = (Cell::new(Data::new(0)), BorrowFlag::new());
    ///
    /// if item.1.can_borrow() {
    ///     unsafe {
    ///         item.1.borrow();
    ///         check_something(item.0.as_ref());
    ///     }
    /// }
    ///
    /// fn check_something(val: &Data) {
    ///     assert_eq!(val.value(), 0)
    /// }
    /// ```
    #[inline]
    pub unsafe fn borrow(&self) {
        *self.inner.as_mut() += 1;
    }
    /// Marks the end of an immutable borrow session or
    ///
    /// Tells the BorrowFlag that an immutable borrow has been dropped
    ///
    /// # Safety
    /// This function is marked `unsafe` because the caller must follow-up or preceed a call to this function by
    /// actually `dropping` the `immutably borrow` or by guarranteeing that the `immutable borrow` will be
    /// dropped before the data is borrowed `mutably` or `taken` ownership  of.

    #[inline]
    pub unsafe fn drop_borrow(&self) {
        *self.inner.as_mut() -= 1;
    }

    /// Marks the start of a mutable borrow session or
    ///
    /// Tells the `BorrowFlag` that a mutable borrow session has started
    ///
    ///# Safety 2
    /// This function is marked unsafe because the caller must guarrantee the following:
    ///
    /// * The data can currently be borrowed mutably or that the call to `can_borrow_mut()` returns true
    /// * The data is actually borrowed mutably corresponding with a call to `borrow_mut()`
    ///
    /// # Example
    ///
    /// ```
    /// use speedy_refs::{Cell, BorrowFlag};
    /// struct Data(usize);
    ///
    /// impl Data {
    ///     fn new(val: usize) -> Self {
    ///         Self(val)
    ///     }
    ///     fn value(&self) -> usize {
    ///         self.0
    ///     }
    ///
    ///     fn increment(&mut self) {
    ///         self.0 += 1;
    ///     }
    /// }
    ///
    /// let item = (Cell::new(Data::new(0)), BorrowFlag::new());
    ///
    /// if item.1.can_borrow_mut() {
    ///     unsafe {
    ///         item.1.borrow_mut();
    ///         check_something(item.0.as_mut());
    ///     }
    /// }
    ///
    /// fn check_something(val: &mut Data) {
    ///     val.increment();
    ///     assert_eq!(val.value(), 1)
    /// }
    /// ```
    #[inline]
    pub unsafe fn borrow_mut(&self) {
        *self.inner.as_mut() = -1;
    }
    /// Marks the end of a mutable borrow session or
    ///
    /// Tells the BorrowFlag that the data is no longer being borrowed mutably
    ///
    /// # Safety
    /// This function is marked `unsafe` because the caller must follow-up or preceed a call to this function by
    /// actually `dropping` the `mutable borrow` or by guarranteeing that the `mutable borrow` will be
    /// dropped before the data is borrowed again or `taken` ownership of
    #[inline]
    pub unsafe fn drop_borrow_mut(&self) {
        *self.inner.as_mut() = 0;
    }
}

/// Since BorrowFlag wraps a `Cell<isize>`,
/// Dropping it should deallocate the `isize`
impl Drop for BorrowFlag {
    fn drop(&mut self) {
        unsafe { self.inner.deallocate() }
    }
}

/// An immutable borrow of RefCell
pub struct Ref<'a, T> {
    cell: &'a RefCell<T>,
}

/// Dropping a Ref object
impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        unsafe { self.cell.flag.drop_borrow() }
    }
}

impl<'a, T> std::ops::Deref for Ref<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // Since self is the only mutable reference to the RefCell,
        // we can create references without minding to count it in the borrow flag
        self.as_ref()
    }
}

impl<'a, T> AsRef<T> for Ref<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        unsafe { self.cell.inner.as_ref() }
    }
}

/// # RefMut
/// A mutable borrow of a RefCell
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
        unsafe { self.cell.flag.drop_borrow_mut() }
    }
}

impl<'a, T> std::ops::Deref for RefMut<'a, T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        // Since self is the only mutable reference to the RefCell,
        // we can create references without minding to count it in the borrow flag
        self.as_ref()
    }
}

impl<'a, T> AsRef<T> for RefMut<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        unsafe { self.cell.inner.as_ref() }
    }
}

impl<'a, T> std::ops::DerefMut for RefMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<'a, T> AsMut<T> for RefMut<'a, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.cell.inner.as_mut() }
    }
}

/// # RefCell
/// A RefCell is a mutable memory location with dynamically checked borrow rules.
///
/// It is similar to `Cell<T>`, but provides the ability to perform mutable borrows and immutable borrows safely
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
/// The `take` method can be used to extract the value from the `RefCell`
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
    inner: Cell<T>,
    flag: BorrowFlag,
}

impl<T> RefCell<T> {
    /// Creates a new RefCell wrapping the given value
    pub fn new(val: T) -> Self {
        Self {
            inner: Cell::new(val),
            flag: BorrowFlag::new(),
        }
    }

    /// Creates a new RefCell from the input `Cell` and `BorrowFlag`
    ///
    /// # Safety
    /// Marked unsafe because the safety of the methods defined on `RefCell` will depend on the validity
    /// of the `Cell` and the accuracy of the `BorrowFlag` passed to this function.
    ///
    /// # Example
    /// ```
    /// use speedy_refs::{Cell, RefCell, BorrowFlag};
    ///
    /// let data = Cell::new(3);
    /// let borrow = unsafe { data.as_ref() };
    ///
    /// let bf = BorrowFlag::new();// Initializes borrowFlag with no current borrows
    ///
    /// let rc = unsafe { RefCell::from_parts(data, bf) };
    /// // the following code is invalid because the borrowflag will not accurately account for the immutable borrow `borrow`
    /// // println!("{}", borrow.as_ref());
    /// // uncommenting the previous line will result in Undefined Behaviour
    ///
    /// // Do some stuff with the rc
    /// ```
    pub unsafe fn from_parts(val: Cell<T>, flag: BorrowFlag) -> Self {
        Self { inner: val, flag }
    }
    /// # Ref
    /// Obtains an immutable borrow of the RefCell
    /// 
    /// # Returns
    /// - `Ref<'a, T>` where `'a` is the lifetime of the RefCell
    pub fn borrow<'a>(&'a self) -> Ref<'a, T> {
        if self.flag.can_borrow() {
            unsafe {
                self.flag.borrow();
            }
            Ref { cell: self }
        } else {
            panic!("T cannot be borrowed immutably while T is borrowed mutably")
        }
    }
    /// # Ref
    /// Obtains a mutable borrow of the RefCell
    /// 
    /// # Returns
    /// - `RefMut<'a, T>` where `'a` is the lifetime of the RefCell
    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, T> {
        if self.flag.can_borrow_mut() {
            unsafe {
                self.flag.borrow_mut();
            }
            RefMut { cell: self }
        } else {
            panic!("T cannot be borrowed mutably while T is borrowed mutably or immutably")
        }
    }
    /// Takes ownership of the value contained in the RefCell
    /// 
    /// # Panics
    /// Panics if the value is currently borrowed
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
        unsafe { self.inner.deallocate() }
        // Drops will be called for self.flag and that will deallocate it
    }
}

impl<T: Clone> Clone for RefCell<T> {
    fn clone(&self) -> Self {
        let clone = self.inner.clone_inner();
        unsafe { RefCell::from_parts(clone, BorrowFlag::new()) }
    }
}
