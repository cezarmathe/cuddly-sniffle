//! cuddly-sniffle

#[cfg(feature = "async")]
compile_error!("The async feature is not supported yet.");

#[cfg(feature = "async")]
mod future;
mod raw;

use std::sync::Arc;
use std::sync::Weak;

use self::raw::RawCell;

/// A cuddly-sniffle cell.
pub struct Cell<T> {
    raw: raw::RawCell<T>,
}

impl<T> Cell<T> {
    /// Create a new cell.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { raw: RawCell::new() }
    }

    /// Create a new cell with a value.
    #[inline(always)]
    pub fn with_value(value: T) -> Self {
        Self { raw: RawCell::with_value(value) }
    }

    /// Get the current value of the cell.
    #[cfg(feature = "async")]
    pub async fn get(&self) -> Arc<T> {
        todo!()
    }

    /// Get the current value of the cell.
    #[inline(always)]
    pub async fn get_blocking(&self) -> Arc<T> {
        self.raw.get_blocking()
    }

    /// Get the current value of the cell (as a weak pointer).
    #[cfg(feature = "async")]
    pub async fn get_weak(&self) -> Weak<T> {
        todo!()
    }

    /// Get the current value of the cell (as a weak pointer).
    #[inline(always)]
    pub async fn get_weak_blocking(&self) -> Weak<T> {
        self.raw.get_weak_blocking()
    }

    /// Update the cell, returning the old value.
    #[cfg(feature = "async")]
    pub async fn update(&self, value: T) -> Arc<T> {
        todo!()
    }

    /// Update the cell, returning the old value.
    #[inline(always)]
    pub fn update_blocking(&self, value: T) -> Arc<T> {
        self.raw.update_blocking(value)
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
    /// Update the cell with the default value, returning the old value.
    #[cfg(feature = "async")]
    pub async fn update_with_default(&self) -> Arc<T> {
        todo!()
    }

    /// Update the cell with the default value, returning the old value.
    pub fn update_blocking_with_default(&self) -> Arc<T> {
        self.raw.update_blocking_with_default()
    }
}
