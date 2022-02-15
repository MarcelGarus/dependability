#![no_std]

use alloc::sync::Arc;
use core::cell::Cell;

extern crate alloc;

pub mod priority_queue;
pub mod task;
pub mod time;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
#[macro_export]
macro_rules! spawn {
    ($(($deadline:expr, $task:expr)),+) => {{
        let mut executor = crate::task::executor::Executor::new();
        $(executor.spawn(Task::new($deadline, crate::task::DelayStrategy::ReturnError, $task)));+;
        executor.run()
    }};
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::{
        task::{executor::Executor, noop, sleep, DelayStrategy, Task},
        time::{StdTimer, Timer},
        PartialSink,
    };
    use std::println;

    async fn async_number() -> u32 {
        42
    }

    async fn async_task(number: u8) {
        println!("Hi {}!", number);
        let number = async_number().await;
        noop::noop().await;
        assert_eq!(number, 42)
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_async_await() {
        let mut executor = Executor::new();
        let now = StdTimer.now();

        executor.spawn(Task::new(
            (now + 10).into(),
            DelayStrategy::ReturnError,
            async_task(1),
        ));
        executor.spawn(Task::new(
            (now + 5).into(),
            DelayStrategy::ReturnError,
            async_task(2),
        ));
        executor.spawn(Task::new(
            (now + 9).into(),
            DelayStrategy::ReturnError,
            async_task(3),
        ));
        executor.spawn(Task::new(
            (now + 2).into(),
            DelayStrategy::ReturnError,
            async_task(4),
        ));
        executor.spawn(Task::new(
            (now + 7).into(),
            DelayStrategy::ReturnError,
            async_task(5),
        ));
        executor.spawn(Task::new(
            (now + 7).into(),
            DelayStrategy::ReturnError,
            async_task(5),
        ));

        // let result = finish_in(Duration::new(123), async || {
        //     ...
        // });
        assert!(executor.run().is_ok());
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_macro() {
        let now = StdTimer.now();
        assert!(spawn!(
            ((now + 4).into(), async_task(1)),
            ((now + 2).into(), async_task(2))
        )
        .is_ok());
    }

    async fn pending_task(number: u8) {
        println!("Starting infinite task {}", number);
        std::thread::sleep(std::time::Duration::from_secs(1));
        let () = std::future::pending().await;
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_missing_deadline() {
        assert!(spawn!(((StdTimer.now() + 2).into(), pending_task(1))).is_err());
    }

    async fn long_task(mut seconds: u8) {
        println!("Long task taking {} seconds", seconds);
        while seconds > 0 {
            sleep::sleep(StdTimer, std::time::Duration::new(1, 0)).await;
            seconds -= 1;
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_continue_running_behavior() {
        let mut executor = Executor::new();
        executor.spawn(Task::new(
            1.into(),
            DelayStrategy::ReturnError,
            long_task(10),
        ));
        assert!(executor.run().is_err());

        let mut executor = Executor::new();
        executor.spawn(Task::new(
            1.into(),
            DelayStrategy::ContinueRunning,
            long_task(3),
        ));
        assert!(executor.run().is_ok());
    }

    async fn partial(sink: alloc::sync::Arc<PartialSink<u8>>) {
        let v1 = 9;
        sink.set(v1);
        std::thread::sleep(core::time::Duration::from_secs(10));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_partial() {
        let mut executor = Executor::new();
        let sink = PartialSink::new();
        executor.spawn(Task::new(
            3.into(),
            DelayStrategy::ReturnError,
            partial(sink.clone()),
        ));
        println!("{:?}", executor.run());
        println!("{:?}", sink.get());
        assert_eq!(Some(9), sink.get());
    }

    #[cfg(feature = "std")]
    async fn subtask(id: usize) {
        println!("Running subtask {}...", id);
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::future::pending::<()>().await;
    }

    #[cfg(feature = "std")]
    async fn complex_task(id: usize) {
        println!("Task {} here!", id);
        for _ in 0..8 {
            subtask(id).await;
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_state_machine() {
        let mut executor = Executor::new();
        let now = StdTimer.now();

        executor.spawn(Task::new(
            (now + 5).into(),
            DelayStrategy::ReturnError,
            complex_task(0),
        ));
        executor.spawn(Task::new(
            (now + 4).into(),
            DelayStrategy::ReturnError,
            complex_task(1),
        ));
        executor.run().unwrap();
    }

    #[cfg(feature = "std")]
    async fn wait_task(t: u8, dur: u64) {
        println!("Task {t} running");
        std::thread::sleep(std::time::Duration::from_secs(dur));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_continue_running_queue() {
        let mut exec = Executor::new();

        exec.spawn(Task::new(
            3.into(),
            DelayStrategy::ContinueRunning,
            wait_task(0, 2),
        ));
        exec.spawn(Task::new(
            4.into(),
            DelayStrategy::ReturnError,
            wait_task(1, 2),
        ));

        assert!(exec.run().is_ok())
    }
}
