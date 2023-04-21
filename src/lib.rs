#![feature(negative_impls)]
#![feature(const_trait_impl)]
///! # speedy_refs
///! A collection of useful smart pointers including some alternatives to std smart pointers.
///! 
///! 
///!
///! # FEATURES
///!
///! - **Rc**:
///!  Blazingly fast alternative to the std `Rc` smart pointer.
///!
///!  
///! - **RefCell**:
///!  Blazingly fast alternative to the std `RefCell`.
///!
///!  
///! - **Arc**:
///! Lighter alternative the std `Arc` with equivalent performance
///!
///! 
///! - **HeapCell**:
///! Similar to `NonNull` with simpler type `deallocation` and `dropping`
///!
///! 
///! - **Reon** - Read only static pointer that implements `Sync` and `Send`
///!
///! 
///! - **RcCell** - Simple and more concise version of `Rc<RefCell>`
///!
///! 
///! - **SharedCell**:
///! For Shared ownership without borrow checking.
///!
///! 
///! - **Cell**:
///! A shared ownership reference type that simulates how references are used in languages like
///! go, python, java, etc. Get multiple mutable references simultaneously without borrow checking (!Sync).
///! Cloning the reference only clones the interior value.

mod arc;
mod rc;
mod reon;
mod cell;

pub(crate) mod atomic;

pub use arc::*;
pub use rc::*;
pub use reon::*;
pub use cell::*;