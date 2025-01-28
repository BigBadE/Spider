pub mod spawning;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct PausableFuture<F: Fn(Waker)> {
    pub waker: F,
    pub finishing: bool
}

/// TODO read more if this is safe
impl<F: Fn(Waker)> Unpin for PausableFuture<F> {

}

impl<F: Fn(Waker)> Future for PausableFuture<F> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.finishing {
            return Poll::Ready(());
        }

        self.finishing = true;
        (self.waker)(cx.waker().clone());
        Poll::Pending
    }
}