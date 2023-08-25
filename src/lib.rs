//! This will run when is droped
//!
//! This will run in drop order, that means newest first
//!
//! ## features:
//!
//! - default = ["std"]
//! - std
//!
//! use `default-features=false` if you don't want to not have std

#![cfg_attr(not(feature = "std"), no_std)]

pub struct AfterDrop<F: FnOnce()>(Option<F>);

pub fn defer<F: FnOnce()>(func: F) -> AfterDrop<F> {
    AfterDrop::new(func)
}

impl<F: FnOnce()> AfterDrop<F> {
    #[must_use]
    pub fn new(func: F) -> Self {
        Self(Some(func))
    }
}

impl<F: FnOnce()> Drop for AfterDrop<F> {
    fn drop(&mut self) {
        self.0.take().unwrap()()
    }
}

impl<F: FnOnce()> From<F> for AfterDrop<F> {
    fn from(func: F) -> Self {
        Self::new(func)
    }
}

#[cfg(feature = "std")]
pub struct AfterDropBoxed(Option<Box<dyn FnOnce()>>);

#[cfg(feature = "std")]
impl AfterDropBoxed {
    #[must_use]
    pub fn new<F: FnOnce() + 'static>(func: F) -> Self {
        Self(Some(Box::new(func)))
    }
}

#[cfg(feature = "std")]
impl Drop for AfterDropBoxed {
    fn drop(&mut self) {
        self.0.take().unwrap()()
    }
}

#[cfg(feature = "std")]
impl<F: FnOnce() + 'static> From<F> for AfterDropBoxed {
    fn from(func: F) -> Self {
        Self::new(func)
    }
}
