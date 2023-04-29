/// # Borrow
/// A smart pointer that provides shared ownership of its contained value `T` with interior mutability.
/// 
/// A `Borrow` is a shared reference to `T` that may be used to modify `T`
///
/// This struct is implemented using an `std::rc::Rc` and a `speedy_refs::SharedCell` to provide shared ownership and interior mutability,
/// respectively. It behaves similarly to a `Cell` in that its contents can be modified through a shared reference.
///
/// # Note
/// Does not provide runtime borrow checking. If you want runtime borrow checking use `RcBorrow` instead.
///
/// # Examples
///
/// ```
/// use speedy_refs::Borrow;
///
/// let mut items: Borrow<Vec<String>> = Borrow::new(vec!["Hello".into(), ",".into(), " ".into(), "World".into(), "!".into()]);
///
/// let clone = items.clone();
///
/// // Modify the contents of the original Borrow.
/// items.pop();
///
/// // Collect shared references to the remaining items.
/// let mut refs = items.iter().collect::<Vec<&String>>();
///
/// assert_eq!(clone.len(), 4);
/// ```
pub struct Borrow<T> {
    value: std::rc::Rc<SharedCell<T>>,
}

pub(crate) use std::fmt::{Debug, Display, Formatter, Result};

impl<T: Debug> Debug for Borrow<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self.get_ref(), f)
    }
}

impl<T: Default> Default for Borrow<T> {
    fn default() -> Self {
        Borrow::new(T::default())
    }
}

impl<T: Display> Display for Borrow<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(self.get_ref(), f)
    }
}

impl<T> AsRef<T> for Borrow<T> {
    fn as_ref(&self) -> &T {
        self.get_ref()
    }
}

impl<T> AsMut<T> for Borrow<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

pub(crate) use std::ops::{Deref, DerefMut};

impl<T> Deref for Borrow<T> {
    type Target = T;

    /// Returns a shared reference to the contained value.
    ///
    /// This method uses the `UnsafeBorrow` internally to allow for shared access without violating Rust's borrowing rules.
    /// However, it is marked as unsafe for the same reason as `SharedCell::as_ref`.
    fn deref(&self) -> &Self::Target {
        self.get_ref()
    }
}

impl<T> DerefMut for Borrow<T> {
    /// Returns a mutable reference to the contained value.
    ///
    /// This method uses the `UnsafeBorrow` internally to allow for mutable access without violating Rust's borrowing rules.
    /// However, it is marked as unsafe for the same reason as `SharedCell::as_mut`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> Borrow<T> {
    /// Creates a new `Borrow` instance with the specified initial value.
    pub fn new(value: T) -> Borrow<T> {
        Self {
            value: std::rc::Rc::new(SharedCell::new(value)),
        }
    }

    pub(crate) fn get_ref(&self) -> &T {
        unsafe { self.value.get_ref() }
    }

    pub(crate) fn get_mut(&self) -> &mut T {
        unsafe { self.value.get_mut() }
    }
}

impl<T: PartialEq> PartialEq for Borrow<T> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.get_ref(), other.get_ref())
    }
    fn ne(&self, other: &Self) -> bool {
        PartialEq::ne(self.get_ref(), other.get_ref())
    }
}

impl<T: Eq> Eq for Borrow<T> {}

impl<T: PartialOrd> PartialOrd for Borrow<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_ref().partial_cmp(other.get_ref())
    }

    fn lt(&self, other: &Self) -> bool {
        self.get_ref().lt(other.get_ref())
    }

    fn le(&self, other: &Self) -> bool {
        self.get_ref().le(other.get_ref())
    }

    fn gt(&self, other: &Self) -> bool {
        self.get_ref().gt(other.get_ref())
    }

    fn ge(&self, other: &Self) -> bool {
        self.get_ref().ge(other.get_ref())
    }
}

impl<T: Ord> Ord for Borrow<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_ref().cmp(other.get_ref())
    }
    // Other things to do
    // Implement default members to refer to the getRef
}

pub(crate) use std::hash::{Hash, Hasher};
impl<T: Hash> Hash for Borrow<T> {
    fn hash<'a, H: Hasher>(&'a self, state: &mut H) {
        self.get_ref().hash(state)
    }
}

impl<T> Clone for Borrow<T> {
    /// Returns a new `Borrow` instance with a shared reference to the same contained value.
    ///
    /// The contents of the `Borrow` can be modified through any of its clones, but each clone still owns a shared reference
    /// to the same value. When all clones are dropped, the value will be dropped as well.
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

impl<T> From<T> for Borrow<T> {
    fn from(value: T) -> Self {
        Borrow::new(value)
    }
}

pub(crate) use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::SharedCell;

impl<T: Serialize> Serialize for Borrow<T> {
    fn serialize<S: Serializer>(&self, sz: S) -> std::result::Result<S::Ok, S::Error> {
        T::serialize(self.get_ref(), sz)
    }
}

impl<'d, T: Deserialize<'d>> Deserialize<'d> for Borrow<T> {
    fn deserialize<D: Deserializer<'d>>(dz: D) -> std::result::Result<Self, D::Error> {
        let value = T::deserialize(dz)?;
        Ok(Borrow::new(value))
    }
}

impl<T> !Send for Borrow<T> {}
impl<T> !Sync for Borrow<T> {}