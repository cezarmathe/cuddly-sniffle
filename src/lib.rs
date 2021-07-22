//! cuddly-sniffle

mod future;
mod raw;

use std::sync::Arc;
use std::sync::Weak;

/// A cuddly-sniffle cell.
pub struct Cell<T> {
    raw: raw::RawCell<T>,
}

impl<T> Cell<T> {
    // pub const fn const_new() -> Self {
    //     todo!()
    // }

    pub fn new() -> Self {
        todo!()
    }

    pub fn with_value(value: T) -> Self {
        todo!()
    }

    pub async fn get(&self) -> Arc<T> {
        todo!()
    }

    pub async fn get_blocking(&self) -> Arc<T> {
        todo!()
    }

    pub async fn get_weak(&self) -> Weak<T> {
        todo!()
    }

    pub async fn get_weak_blocking(&self) -> Weak<T> {
        todo!()
    }

    pub fn update(&self, value: T) {
        todo!()
    }

    pub fn update_blocking(&self, value: T) {
        todo!()
    }
}
