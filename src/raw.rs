//! The raw cuddly-sniffle cell.

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;

use parking_lot::Mutex;
use parking_lot::RwLock;

/// Raw cuddly-sniffle cell.
pub(crate) struct RawCell<T> {
    inner: [RwLock<Option<Arc<T>>>; 2],
    selector: AtomicUsize,
    update_lock: Mutex<()>,
}

impl<T: Default> Default for RawCell<T> {
    fn default() -> Self {
        Self {
            inner: [
                RwLock::new(Some(Arc::new(T::default()))),
                RwLock::new(None),
            ],
            selector: AtomicUsize::new(0),
            update_lock: Mutex::new(()),
        }
    }
}

impl<T> RawCell<T> {
    /// Create a new, empty RawCell.
    pub const fn new() -> Self {
        Self {
            inner: [
                parking_lot::const_rwlock(None),
                parking_lot::const_rwlock(None),
            ],
            selector: AtomicUsize::new(0),
            update_lock: parking_lot::const_mutex(()),
        }
    }

    /// Create a new RawCell with a value.
    pub fn with_value(value: T) -> Self {
        Self {
            inner: [
                RwLock::new(Some(Arc::new(value))),
                RwLock::new(None),
            ],
            selector: AtomicUsize::new(0),
            update_lock: Mutex::new(()),
        }
    }

    /// Get the current value of the cache.
    pub fn get(&self) -> Arc<T> {
        loop {
            let selector = self.get_selector();
            let cache = self.inner[selector].read();
            if let Some(value) = cache.as_ref() {
                break value.clone()
            }
        }
    }

    /// Update the cell, returning the old value.
    pub fn update(&self, new: T) -> Arc<T> {
        let _ = self.update_lock.lock();
        let selector = self.get_selector();
        {
            let mut cell = self.inner[selector ^ 1].write();
            *cell = Some(Arc::new(new));
        }
        self.switch_selector();
        {
            let mut cell = self.inner[selector].write();
            cell.take().expect("RawCell::update - old value was missing from the cell")
        }
    }

    #[inline]
    fn get_selector(&self) -> usize {
        self.selector.load(AtomicOrdering::SeqCst)
    }

    #[inline]
    fn switch_selector(&self) {
        self.selector.fetch_xor(1, AtomicOrdering::SeqCst);
    }
}

impl<T: Default> RawCell<T> {
    /// Update the cell with the default value, returning the old value.
    pub fn update_with_default(&self) -> Arc<T> {
        self.update(T::default())
    }
}
