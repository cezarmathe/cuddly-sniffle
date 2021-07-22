//! Futures for cuddly-sniffle.

use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

/// A future that returns a value from the cell.
pub struct GetFuture<T> {
    _phantom: PhantomData<T>,
}

pub enum GetFutureState<T> {
    Done(T),
}

impl<T> Future for GetFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

/// A future that updates the cell with a new value.
pub struct UpdateFuture<T> {
    _phantom: PhantomData<T>,
}
