//! # qrwcell - quick read-write cell
//!
//! Read-write cell that aims to reduce the amount of blocking compared to a
//! single read-write lock.
//!
//! The cell has two slots - one for reading and one for writing. Writing
//! alternates the slot that is currently served to readers, thereby minimising
//! blocking on a reader-writer lock.

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
    ///
    /// This function will panic if the cell is in an uninitialized state.
    #[inline(always)]
    #[must_use]
    pub fn get(&self) -> Arc<T> {
        self.raw.get()
    }

    /// Get the current value of the cell (as a weak pointer).
    ///
    /// This function will panic if the cell is in an uninitialized state.
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

#[cfg(test)]
mod test {
    use crate::QrwCell;

    #[test]
    #[should_panic]
    fn test_new_get_panic() {
        let cell: QrwCell<Vec<u32>> = QrwCell::new();
        let _ = cell.get();
    }

    #[test]
    #[should_panic]
    fn test_new_get_weak_panic() {
        let cell: QrwCell<Vec<u32>> = QrwCell::new();
        let _ = cell.get_weak();
    }

    #[test]
    fn test_default_get() {
        let cell: QrwCell<Vec<u32>> = QrwCell::default();
        assert_eq!(<Vec<u32>>::default(), *cell.get())
    }

    #[test]
    fn test_default_get_weak() {
        let cell: QrwCell<Vec<u32>> = QrwCell::default();
        assert_eq!(
            <Vec<u32>>::default(),
            *cell
                .get_weak()
                .upgrade()
                .expect("Weak pointer missing value")
        )
    }

    #[test]
    fn test_with_value_get() {
        let cell = QrwCell::with_value(vec![42, 42, 42, 42]);
        assert_eq!(vec![42, 42, 42, 42], *cell.get())
    }

    #[test]
    fn test_with_value_get_weak() {
        let cell: QrwCell<Vec<u32>> = QrwCell::with_value(vec![42, 42, 42, 42]);
        assert_eq!(
            vec![42, 42, 42, 42],
            *cell
                .get_weak()
                .upgrade()
                .expect("Weak pointer missing value")
        )
    }

    #[test]
    fn test_update() {
        let cell = QrwCell::new();

        let vec = vec![0, 0, 0, 0];
        let vec_clone = vec.clone();
        assert_eq!(*cell.update(vec), vec_clone);

        let new_vec = vec![42, 0, 0, 0];
        let new_vec_clone = new_vec.clone();
        assert_eq!(*cell.update(new_vec), vec_clone);
        assert_eq!(*cell.get(), new_vec_clone);
    }

    #[test]
    fn test_update_default() {
        let cell = QrwCell::new();

        assert_eq!(*cell.update(vec![0, 0, 0, 0]), vec![0, 0, 0, 0]);

        assert_eq!(*cell.update_with_default(), vec![0, 0, 0, 0]);
        assert_eq!(*cell.get(), Vec::default());
    }

    #[test]
    fn test_get_weak_update() {
        let cell = QrwCell::with_value(vec![42, 42, 42, 42]);
        assert_eq!(*cell.get_weak().upgrade().unwrap(), vec![42, 42, 42, 42]);

        let weak = cell.get_weak();
        cell.update_with_default();
        assert_eq!(weak.upgrade(), None);
    }
}
