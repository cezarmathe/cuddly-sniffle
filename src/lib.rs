//! # qrwcell - quick read-write cell
//!
//! Read-write cell that aims to reduce the amount of blocking compared to a
//! single read-write lock.
//!
//! The cell has two slots - one for reading and one for writing. Writing
//! alternates the slot that is currently served to readers, thereby minimising
//! blocking on a reader-writer lock.
//!
//! Please be aware that if a cell is not created with a value or updated at
//! least once attempting to get the inner value will loop forever!


mod raw;

use std::sync::Arc;
use std::sync::Weak;

use self::raw::RawQrwCell;

/// A quick read-write cell.
pub struct QrwCell<T> {
    raw: RawQrwCell<T>,
}

impl<T> QrwCell<T> {
    /// Create a new cell.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            raw: RawQrwCell::new(),
        }
    }

    /// Create a new cell with a value.
    #[inline(always)]
    pub fn with_value(value: T) -> Self {
        Self {
            raw: RawQrwCell::with_value(value),
        }
    }

    /// Get the current value of the cell.
    #[inline(always)]
    #[must_use]
    pub fn get(&self) -> Arc<T> {
        self.raw.get()
    }

    /// Get the current value of the cell (as a weak pointer).
    #[inline(always)]
    pub fn get_weak(&self) -> Weak<T> {
        self.raw.get_weak()
    }

    /// Update the cell, returning the old value.
    ///
    /// If the cell did not previously have a value, the same value is returned.
    #[inline(always)]
    pub fn update(&self, value: T) -> Arc<T> {
        self.raw.update(value)
    }
}

impl<T: Default> Default for QrwCell<T> {
    fn default() -> Self {
        Self {
            raw: RawQrwCell::default(),
        }
    }
}

impl<T: Default> QrwCell<T> {
    /// Update the cell with the default value for this type,
    /// returning the old value.
    #[inline(always)]
    pub fn update_with_default(&self) -> Arc<T> {
        self.update(T::default())
    }
}
