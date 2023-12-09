//! This will run when is droped
//!
//! This will run in drop order, that means newest first
//!
//! ## features:
//!
//! - default = ["std"]
//! - std
//!
//! use `default-features=false` if you don't want std

#![cfg_attr(not(feature = "std"), no_std)]

/// A simple continer for the defer function that is called on drop!

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

/// This is a boxed version to be stored easier!

#[cfg(feature = "std")]
pub struct AfterDropBoxed<'a>(Option<Box<dyn FnOnce() + 'a>>);

#[cfg(feature = "std")]
impl<'a> AfterDropBoxed<'a> {
    #[must_use]
    pub fn new<F: FnOnce() + 'a>(func: F) -> Self {
        Self(Some(Box::new(func)))
    }
}

#[cfg(feature = "std")]
impl<'a> Drop for AfterDropBoxed<'a> {
    fn drop(&mut self) {
        self.0.take().unwrap()()
    }
}

#[cfg(feature = "std")]
impl<'a, F: FnOnce() + 'a> From<F> for AfterDropBoxed<'a> {
    fn from(func: F) -> Self {
        Self::new(func)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn defer() {
        let mut droped = false;

        let _defer = crate::defer(|| droped = true);
        drop(_defer);

        assert!(droped)
    }

    #[test]
    fn drop_after_boxed() {
        let mut droped = false;

        let _boxed = crate::AfterDropBoxed::new(|| droped = true);
        drop(_boxed);

        assert!(droped);
    }
}
