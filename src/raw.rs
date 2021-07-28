//! The raw quick read-write cell.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::sync::Arc;
use std::sync::Weak;

use parking_lot::Mutex;
use parking_lot::Once;
use parking_lot::OnceState;
use parking_lot::RwLock;

/// Raw quick read-write cell.
pub(crate) struct RawQrwCell<T> {
    inner: [RwLock<Option<Arc<T>>>; 2],
    selector: AtomicUsize,
    update_lock: Mutex<()>,
    init: Once,
}

impl<T: Default> Default for RawQrwCell<T> {
    fn default() -> Self {
        let init = Once::new();
        init.call_once(|| ());
        Self {
            inner: [RwLock::new(Some(Arc::new(T::default()))), RwLock::new(None)],
            selector: AtomicUsize::new(0),
            update_lock: Mutex::new(()),
            init,
        }
    }
}

impl<T> RawQrwCell<T> {
    /// Create a new, empty RawQrwCell.
    pub(crate) const fn new() -> Self {
        Self {
            inner: [
                parking_lot::const_rwlock(None),
                parking_lot::const_rwlock(None),
            ],
            selector: AtomicUsize::new(0),
            update_lock: parking_lot::const_mutex(()),
            init: Once::new(),
        }
    }

    /// Create a new RawQrwCell with a value.
    pub(crate) fn with_value(value: T) -> Self {
        let init = Once::new();
        init.call_once(|| ());
        Self {
            inner: [RwLock::new(Some(Arc::new(value))), RwLock::new(None)],
            selector: AtomicUsize::new(0),
            update_lock: Mutex::new(()),
            init,
        }
    }

    /// Get the current value of the cell.
    pub(crate) fn get(&self) -> Arc<T> {
        if self.init.state() != OnceState::Done {
            panic!("Attempted to read from a qrwcell that has not been initialized!");
        }
        loop {
            let selector = self.get_selector();
            let guard = self.inner[selector].read();
            if let Some(value) = guard.as_ref() {
                break Arc::clone(value);
            }
        }
    }

    /// Get the current value of the cell (as a weak pointer).
    pub(crate) fn get_weak(&self) -> Weak<T> {
        if self.init.state() != OnceState::Done {
            panic!("Attempted to read from a qrwcell that has not been initialized!");
        }
        loop {
            let selector = self.get_selector();
            let guard = self.inner[selector].read();
            if let Some(value) = guard.as_ref() {
                break Arc::downgrade(value);
            }
        }
    }

    /// Update the cell, returning the old value.
    pub(crate) fn update(&self, new: T) -> Arc<T> {
        self.init.call_once(|| ());
        let _update_lock = self.update_lock.lock();
        let new_arc = Arc::new(new);
        let selector = self.get_selector() ^ 1;
        {
            let mut cell = self.inner[selector].write();
            *cell = Some(Arc::clone(&new_arc));
        }
        let selector = self.switch_selector();
        {
            let mut cell = self.inner[selector].write();
            cell.take().unwrap_or(new_arc)
        }
    }

    #[inline]
    fn get_selector(&self) -> usize {
        self.selector.load(AtomicOrdering::Acquire)
    }

    #[inline]
    fn switch_selector(&self) -> usize {
        self.selector.fetch_xor(1, AtomicOrdering::Release)
    }
}
