use core::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use futures_util::Future;

use crate::time::{Timer, Timestamp};

pub fn sleep_until<T: Timer>(timer: T, deadline: Timestamp) -> Sleep<T> {
    Sleep::new(timer, deadline)
}

pub fn sleep<T: Timer>(timer: T, duration: Duration) -> Sleep<T> {
    let deadline = timer.now() + duration.as_secs();
    Sleep::new(timer, deadline)
}

pub struct Sleep<T: Timer> {
    timer: T,
    deadline: Timestamp,
}

impl<T: Timer> Sleep<T> {
    fn new(timer: T, deadline: Timestamp) -> Self {
        Sleep { timer, deadline }
    }
}

impl<T: Timer> Future for Sleep<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.timer.now() > self.deadline {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
