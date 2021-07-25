//! cuddly-sniffle

mod raw;

use std::sync::Arc;
use std::sync::Weak;

use self::raw::RawCell;

/// A cuddly-sniffle cell.
pub struct Cell<T> {
    raw: RawCell<T>,
}

impl<T> Cell<T> {
    /// Create a new cell.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            raw: RawCell::new(),
        }
    }

    /// Create a new cell with a value.
    #[inline(always)]
    pub fn with_value(value: T) -> Self {
        Self {
            raw: RawCell::with_value(value),
        }
    }

    /// Get the current value of the cell.
    #[inline(always)]
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

impl<T: Default> Default for Cell<T> {
    fn default() -> Self {
        Self {
            raw: RawCell::default(),
        }
    }
}

impl<T: Default> Cell<T> {
    /// Update the cell with the default value for this type,
    /// returning the old value.
    #[inline(always)]
    pub fn update_with_default(&self) -> Arc<T> {
        self.update(T::default())
    }
}
