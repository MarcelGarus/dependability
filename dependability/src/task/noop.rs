use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::Future;

pub fn noop() -> impl Future<Output = ()> {
    Noop::new()
}

pub struct Noop {
    waited: bool,
}

impl Noop {
    fn new() -> Self {
        Self { waited: false }
    }
}

impl Future for Noop {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.waited {
            Poll::Ready(())
        } else {
            self.waited = true;
            Poll::Pending
        }
    }
}
