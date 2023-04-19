#![feature(negative_impls)]

mod arc;
mod rc;
mod reon;
mod cell;

pub(crate) mod atomic;

pub use arc::*;
pub use rc::*;
pub use reon::*;
pub use cell::*;

pub struct RcCell<T> {
    inner: std::rc::Rc<std::cell::RefCell<T>>,
}

impl<T> RcCell<T> {
    pub fn new(value: T) -> RcCell<T>{
        Self {
            inner: std::rc::Rc::new(std::cell::RefCell::new(value)),
        }
    }
}

impl<T> Clone for RcCell<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T> std::ops::Deref for RcCell<T> {
    type Target = std::cell::RefCell<T>;
    fn deref(&self) -> &Self::Target {
        std::ops::Deref::deref(&self.inner)
    }
}