use std::ops::{Deref, DerefMut};

/// A container type that allows mutation of its contents even when it is
/// externally immutable.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Cell<T> {
    inner: *mut T,
}

impl<T> Cell<T> {
    /// Creates a new `Cell` containing the given value.
    ///
    /// # Safety
    ///
    /// This function takes ownership of the given value and stores it behind a
    /// raw pointer. It is the responsibility of the caller to ensure that the
    /// value passed in is valid and the Cell is not used after the value has been
    /// dropped or moved.
    ///
    /// # Panics
    ///
    /// This function will panic if memory allocation fails.
    pub fn new(val: T) -> Self {
        Self {
            inner: Box::into_raw(Box::new(val)),
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
        *Box::from_raw(self.inner)
    }

    /// Deallocates the memory of `self.inner`
    ///
    /// This function first calls drop on T and then deallocates it
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `Cell` is not used after calling `deallocate` as
    /// `self.inner` will then point to an invalid memory.
    pub unsafe fn deallocate(&self) {
        let ptr = self.inner;
        std::ptr::drop_in_place(ptr);
        std::alloc::dealloc(ptr as *mut u8, std::alloc::Layout::new::<T>());
    }
}


/// # Borrow
/// A simple struct for keeping track of reads and write to an owned value.
/// 
/// Rust rules allow a number of reads or a single write at a time. 
/// 
/// * `Borrow.read` stores the number of reads that are currently in use 
/// while 
/// * `Borrow.write` tells if there is currenly a reader of the value.
/// 
/// # Note
/// Borrow is meant to be added as a field in your struct for added functionalities since it
/// stores no value within it.
/// 
pub struct Borrow {
    read: usize,
    write: bool,
}

impl Borrow {
    #[inline]
    pub fn new() -> Self {
        Self {
            read: 0,
            write: false,
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
        self.write == false
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
        self.write == false && self.read == 0
    }

    /// Marks a the start of a read by a new reader
    /// 
    /// It does this by increases the count of the `read` field
    /// 
    #[inline]
    pub fn read(&mut self) {
        self.read += 1;
    }

    /// Marks the end of a read by an existing reader
    /// 
    /// It does this by decrementing the count of the `read` field
    /// 
    #[inline]
    pub fn unread(&mut self) {
        self.read -= 1;
    }

    /// Marks the start of a write
    /// 
    /// Sets the `write` field to true
    #[inline]
    pub fn write(&mut self) {
        self.write = true;
    }

    /// Marks the end of a write
    /// 
    /// Sets the `write` field to false
    #[inline]
    pub fn unwrite(&mut self) {
        self.write = false;
    }
}

/// # BorrowFlag
/// For `immutably` and `safely` tracking the reads and writes to an owned value.
/// 
/// It makes use of its inner value Cell<Borrow>.
/// It basically makes unsafe calls to the value inside of inner
#[repr(transparent)]
pub struct BorrowFlag {
    inner: Cell<Borrow>,
}

impl BorrowFlag {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Cell::new(Borrow::new()),
        }
    }

    #[inline]
    pub fn can_borrow(&self) -> bool {
        unsafe { self.inner.as_ref().can_borrow() }
    }

    #[inline]
    pub fn can_borrow_mut(&self) -> bool {
        unsafe { self.inner.as_ref().can_borrow_mut() }
    }
    #[inline]
    pub fn read(&self) {
        unsafe { self.inner.as_mut().read() }
    }

    #[inline]
    pub fn unread(&self) {
        unsafe { self.inner.as_mut().unread() }
    }

    #[inline]
    pub fn write(&self) {
        unsafe { self.inner.as_mut().write() }
    }

    #[inline]
    pub fn unwrite(&self) {
        unsafe { self.inner.as_mut().unwrite() }
    }
}

impl Drop for BorrowFlag {
    fn drop(&mut self) {
        unsafe { self.inner.deallocate() }
    }
}

pub struct Ref<'a, T> {
    cell: &'a RefCell<T>,
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        self.cell.flag.unread()
    }
}

impl<'a, T> Deref for Ref<'a, T> {
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
        self.deref()
    }
}

pub struct RefMut<'a, T> {
    cell: &'a RefCell<T>,
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        self.cell.flag.unwrite()
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
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
        self.deref()
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.cell.inner.as_mut() }
    }
}

impl<'a, T> AsMut<T> for RefMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

pub struct RefCell<T> {
    inner: Cell<T>,
    flag: BorrowFlag,
}

impl<T> RefCell<T> {
    pub fn new(val: T) -> Self {
        Self {
            inner: Cell::new(val),
            flag: BorrowFlag::new(),
        }
    }

    pub fn borrow<'a>(&'a self) -> Ref<'a, T> {
        if self.flag.can_borrow() {
            self.flag.read();
            Ref { cell: self }
        } else {
            panic!("T cannot be borrowed immutably while T is already borrowed mutably")
        }
    }

    pub fn borrow_mut<'a>(&'a self) -> RefMut<'a, T> {
        if self.flag.can_borrow_mut() {
            self.flag.write();
            RefMut { cell: self }
        } else {
            panic!("T cannot be borrowed mutably while T is already borrowed mutably or immutably")
        }
    }

    pub fn take(self) -> T {
        unsafe { self.inner.take() }
    }
}

impl<T> Drop for RefCell<T> {
    fn drop(&mut self) {
        unsafe { self.inner.deallocate() }
    }
}
