use alloc::boxed::Box;
//use core::sync::atomic::{AtomicU64, Ordering};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use crate::time::Timestamp;

pub mod executor;
pub mod noop;
pub mod sleep;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        //static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        //TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
        static mut NEXT_ID: u64 = 0;
        let id = unsafe { TaskId(NEXT_ID) };
        unsafe { NEXT_ID += 1 };
        id
    }
}

pub struct Task {
    id: TaskId,
    deadline: Timestamp,
    behavior: DelayStrategy,
    future: Pin<Box<dyn Future<Output = ()>>>,
}
pub enum DelayStrategy {
    ReturnError,
    Panic,
    ContinueRunning,
    InsteadApproximate(Box<dyn Fn() -> Task>),
}

impl Task {
    pub fn new(
        deadline: Timestamp,
        behavior: DelayStrategy,
        future: impl Future<Output = ()> + 'static,
    ) -> Task {
        Task {
            id: TaskId::new(),
            deadline,
            behavior,
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
