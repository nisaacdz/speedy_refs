#![feature(negative_impls)]
#![feature(const_trait_impl)]
//! # speedy_refs
//! A collection of useful smart pointers including some alternatives to std smart pointers.
//! 
//! 
//!
//! # FEATURES
//!
//! - **Rc**:
//!  Blazingly fast alternative to the std `Rc` smart pointer.
//!
//!  
//! - **RefCell**:
//!  Blazingly fast alternative to the std `RefCell`.
//!
//!  
//! - **Arc**:
//! Lighter alternative the std `Arc` with equivalent performance
//!
//! 
//! - **HeapCell**:
//! Similar to `NonNull` with simpler type `deallocation` and `dropping`
//!
//! 
//! - **Reon** - Read only static pointer that implements `Sync` and `Send`
//!
//! 
//! - **RcCell** - Simple and more concise version of `Rc<RefCell>`
//!
//! 
//! - **SharedCell**:
//! For Shared ownership without borrow checking.
//!
//! 
//! - **Borrow**:
//! A cloneable shared ownership without borrow checking. Like how references are used in languages like java, go, python, etc.

mod arc;
mod rc;
mod reon;
mod cell;
mod borrow;

pub(crate) mod atomic;

pub use arc::*;
pub use rc::*;
pub use reon::*;
pub use cell::*;
pub use borrow::*;

#[cfg(test)]
mod test;