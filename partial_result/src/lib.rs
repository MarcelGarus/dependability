#![no_std]
#![feature(async_closure)]

extern crate alloc;

use alloc::rc::Rc;
use core::cell::Cell;
use dependability_runtime::{
    task::{deadline::Deadline, executor::Executor, DelayStrategy, Task},
    time::Timer,
};
use futures_util::Future;

pub struct PartialResultInput<T> {
    inner: Rc<Cell<Option<T>>>,
}
impl<T> PartialResultInput<T> {
    pub fn set(&self, value: T) {
        self.inner.set(Some(value));
    }
}

pub struct PartialResultOutput<T> {
    inner: Rc<Cell<Option<T>>>,
}
impl<T> PartialResultOutput<T> {
    pub fn get(self) -> Option<T> {
        self.inner.take()
    }
}

pub fn partial_result_slot<T>() -> (PartialResultInput<T>, PartialResultOutput<T>) {
    let rc = Rc::new(Cell::new(None));
    (
        PartialResultInput { inner: rc.clone() },
        PartialResultOutput { inner: rc },
    )
}

pub trait ExecutorExt {
    fn spawn_partial<R, F>(
        &mut self,
        deadline: Deadline,
        closure: fn(PartialResultInput<R>) -> F,
    ) -> PartialResultOutput<R>
    where
        R: 'static,
        F: Future<Output = ()> + 'static;
}
impl<T: Timer> ExecutorExt for Executor<T> {
    fn spawn_partial<R, F>(
        &mut self,
        deadline: Deadline,
        closure: fn(PartialResultInput<R>) -> F,
    ) -> PartialResultOutput<R>
    where
        R: 'static,
        F: Future<Output = ()> + 'static,
    {
        let (input, output) = partial_result_slot();

        self.spawn(Task::new(
            deadline,
            DelayStrategy::SilentlyAbort,
            async move {
                closure(input).await;
            },
        ));

        output
    }
}

mod tests {
    #[cfg(feature = "std")]
    #[test]
    fn test_partial_results() {
        extern crate std;

        use std::println;

        use dependability_runtime::{
            task::{executor::Executor, noop::noop},
            time::{StdTimer, Timer},
        };

        use crate::ExecutorExt;

        let mut executor = Executor::new();
        let now = StdTimer.now();

        let output = executor.spawn_partial::<i32, _>(now.into(), async move |slot| {
            slot.set(39);
            noop().await;
            slot.set(44);
            noop().await;
            slot.set(42);
        });

        executor.run().unwrap();

        println!("The answer is {}.", output.get().unwrap_or(0))
    }
}
