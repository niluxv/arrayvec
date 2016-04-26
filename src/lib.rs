//! **arrayvec** provides the types `ArrayVec` and `ArrayString`: 
//! array-backed vector and string types, which store their contents inline.
//!
//! The **arrayvec** crate has the following cargo feature flags:
//!
//! - `std`
//!   - Optional, enabled by default
//!   - Requires Rust 1.6 *to disable*
//!   - Use libstd
//!
//! - `use_union`
//!   - Optional
//!   - Requires Rust nightly channel
//!   - Use the unstable feature untagged unions for the internal implementation,
//!     which has reduced space overhead
#![cfg_attr(not(feature="std"), no_std)]

extern crate odds;
extern crate nodrop;

#[cfg(not(feature="std"))]
extern crate core as std;

use std::fmt;
#[cfg(feature="std")]
use std::error::Error;
#[cfg(feature="std")]
use std::any::Any; // core but unused

mod array;
mod string;
mod vec;
mod raw;

pub use array::Array;
pub use odds::IndexRange as RangeArgument;
pub use raw::Drain;
pub use string::ArrayString;
pub use vec::ArrayVec;

/// Error value indicating insufficient capacity
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct CapacityError<T = ()> {
    element: T,
}

impl<T> CapacityError<T> {
    fn new(element: T) -> CapacityError<T> {
        CapacityError {
            element: element,
        }
    }

    /// Extract the overflowing element
    pub fn element(self) -> T {
        self.element
    }

    /// Convert into a `CapacityError` that does not carry an element.
    pub fn simplify(self) -> CapacityError {
        CapacityError { element: () }
    }
}

const CAPERROR: &'static str = "insufficient capacity";

#[cfg(feature="std")]
/// Requires `features="std"`.
impl<T: Any> Error for CapacityError<T> {
    fn description(&self) -> &str {
        CAPERROR
    }
}

impl<T> fmt::Display for CapacityError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", CAPERROR)
    }
}

impl<T> fmt::Debug for CapacityError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", "CapacityError", CAPERROR)
    }
}
