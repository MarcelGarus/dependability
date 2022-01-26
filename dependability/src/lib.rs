extern crate alloc;

pub mod priority_queue;
pub mod task;
pub mod time;

pub async fn noop() {}

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
#[macro_export]
macro_rules! spawn {
    ($(($deadline:expr, $task:expr)),+) => {{
        let mut executor = crate::task::executor::Executor::new();
        $(executor.spawn(Task::new($deadline, crate::task::BehaviorWhenDeadlineMissed::ReturnError, $task)));+;
        executor.run()
    }};
}

#[cfg(test)]
mod tests {
    extern crate std;
    use crate::{
        noop,
        task::BehaviorWhenDeadlineMissed,
        task::{executor::Executor, Task},
    };
    use std::println;

    async fn async_number() -> u32 {
        42
    }

    async fn async_task(number: u8) {
        println!("Hi {}!", number);
        let number = async_number().await;
        noop().await;
        assert_eq!(number, 42)
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_async_await() {
        let mut executor = Executor::new();

        executor.spawn(Task::new(
            10,
            BehaviorWhenDeadlineMissed::ReturnError,
            async_task(1),
        ));
        executor.spawn(Task::new(
            5,
            BehaviorWhenDeadlineMissed::ReturnError,
            async_task(2),
        ));
        executor.spawn(Task::new(
            9,
            BehaviorWhenDeadlineMissed::ReturnError,
            async_task(3),
        ));
        executor.spawn(Task::new(
            2,
            BehaviorWhenDeadlineMissed::ReturnError,
            async_task(4),
        ));
        executor.spawn(Task::new(
            7,
            BehaviorWhenDeadlineMissed::ReturnError,
            async_task(5),
        ));
        executor.spawn(Task::new(
            7,
            BehaviorWhenDeadlineMissed::ReturnError,
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
        assert!(spawn!((4, async_task(1)), (2, async_task(2))).is_ok());
    }

    async fn pending_task(number: u8) {
        println!("Starting infinite task {}", number);
        std::thread::sleep(std::time::Duration::from_secs(1));
        let () = std::future::pending().await;
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_missing_deadline() {
        assert!(spawn!((2, pending_task(1))).is_err());
    }

    async fn long_task(number: u8) {
        println!("Hi {}!", number);
        std::thread::sleep(std::time::Duration::new(number.into(), 0));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_continue_running_behavior() {
        let mut executor = Executor::new();
        executor.spawn(Task::new(
            1,
            BehaviorWhenDeadlineMissed::ContinueRunning,
            long_task(1),
        ));
        executor.spawn(Task::new(
            1,
            BehaviorWhenDeadlineMissed::ContinueRunning,
            long_task(2),
        ));
        assert!(executor.run().is_ok());
    }
}
