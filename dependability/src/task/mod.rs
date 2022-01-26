use alloc::boxed::Box;
use core::sync::atomic::{AtomicU64, Ordering};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::time::Timestamp;

pub mod executor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    id: TaskId,
    deadline: Timestamp,
    behavior: BehaviorWhenDeadlinesMissed,
    future: Pin<Box<dyn Future<Output = ()>>>,
}
pub enum BehaviorWhenDeadlinesMissed {
    ReturnError,
    ContinueRunning,
}

impl Task {
    pub fn new(deadline: Timestamp, future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            deadline,
            behavior: BehaviorWhenDeadlinesMissed::ReturnError,
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}