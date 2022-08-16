//!
//! Cross-platform bindings for locking a value into RAM (aka preventing it from getting swapped)
//! 

use std::{
    io,
    ops::{Deref, DerefMut},
};

/// `mlock` container
/// 
/// Note: While you theoretically can swap the values contained inside this struct, you really shouldn't.
/// It won't segfault or anything, it will just return an error upon unlocking.
pub struct Mlock<T> {
    inner: T,
}

impl<T> Mlock<T> {
    /// Construct a new `mlock` wrapper around an item. This constructor executes `mlock` on the value.
    ///
    /// Note: If you pass in a `Box`, this function will lock the `Box` and not the underlying value.
    /// I'm still working on a better API that allows to lock the actual memory range inside the box.
    pub fn new(inner: T) -> io::Result<Self> {
        #[cfg(target_family = "unix")]
        unix::mlock(&inner)?;
        #[cfg(target_family = "windows")]
        windows::mlock(&inner)?;

        Ok(Self { inner })
    }

    /// Remove the memory lock from the address range of the inner item
    ///
    /// The unlock operation might fail. This error is irrecoverable. The item will be lost.
    pub fn unlock(self) -> io::Result<T> {
        #[cfg(target_family = "unix")]
        unix::munlock(&self.inner)?;
        #[cfg(target_family = "windows")]
        windows::munlock(&self.inner)?;

        Ok(self.inner)
    }
}

impl<T> Deref for Mlock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Mlock<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "windows")]
mod windows;
