//! Some caching stuff.
//! 
//! So the use case is kind of like this: you have some read-only assets that 
//! you want to cache in your application and refresh those values every so
//! often (or whenever you want). You don't want your application to get blocked
//! while updating the cache.
//! 
//! Also, seamless accumulators?
//! - sink that appends events
//! - switch to second sink while draining and processing first sink
//! ???


// updatable caches that require minimal time for doing the actual update
// 2-slot cell for appender/batch processor workflows

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;
use std::sync::Arc;

use parking_lot::Mutex;
use parking_lot::RwLock;
use parking_lot::lock_api::GuardSend;
use parking_lot::lock_api::RawRwLock;

// selector: 0 / 1 (cell 0 / cell1)
// cell 0 / 1: idle, read, write

// 0b0000000;

const LOCK_SELECTOR_CELL0: usize = 0b0000000;
const LOCK_SELECTOR_CELL1: usize = 0b0000001;
const LOCK_CELL0_IDLE: usize     = 0b0000010;
const LOCK_CELL0_READ: usize     = 0b0000100;
const LOCK_CELL0_WRITE: usize    = 0b0001000;
const LOCK_CELL1_IDLE: usize     = 0b0010000;
const LOCK_CELL1_READ: usize     = 0b0100000;
const LOCK_CELL1_WRITE: usize    = 0b1000000;

struct CacheLock {
    inner: AtomicUsize,
}

/// A cache.
pub struct Cache<T> {
    inner: [RwLock<Option<Arc<T>>>; 2],
    selector: AtomicUsize,
    update_lock: Mutex<()>,
}

impl<T> Cache<T> {
    /// Create a new cache
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
}

impl<T: Default> Cache<T> {
    /// Create a new cache with an initial value.
    pub fn with_initial_value(value: T) -> Self {
        Self {
            inner: [
                parking_lot::const_rwlock(Some(Arc::new(value))),
                parking_lot::const_rwlock(None),
            ],
            selector: AtomicUsize::new(0),
            update_lock: parking_lot::const_mutex(()),
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
            std::thread::yield_now();
        }
    }

    /// Update the cache.
    pub fn update(&self, new: T) {
        let _ = self.update_lock.lock();
        let selector = self.get_selector();
        {
            let mut cell = self.inner[selector ^ 1].write();
            *cell = Some(Arc::new(new));
        }
        self.switch_selector();
        {
            let mut cell = self.inner[selector].write();
            *cell = None;
        }
    }

    #[inline]
    fn get_selector(&self) -> usize {
        self.selector.load(AtomicOrdering::SeqCst)
    }

    #[inline]
    fn get_and_switch_selector(&self) -> usize {
        todo!()
    }

    #[inline]
    fn switch_selector(&self) {
        self.selector.fetch_xor(1, AtomicOrdering::SeqCst);
    }
}
