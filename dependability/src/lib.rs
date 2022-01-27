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
            now + 10,
            DelayStrategy::ReturnError,
            async_task(1),
        ));
        executor.spawn(Task::new(
            now + 5,
            DelayStrategy::ReturnError,
            async_task(2),
        ));
        executor.spawn(Task::new(
            now + 9,
            DelayStrategy::ReturnError,
            async_task(3),
        ));
        executor.spawn(Task::new(
            now + 2,
            DelayStrategy::ReturnError,
            async_task(4),
        ));
        executor.spawn(Task::new(
            now + 7,
            DelayStrategy::ReturnError,
            async_task(5),
        ));
        executor.spawn(Task::new(
            now + 7,
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
        assert!(spawn!((now + 4, async_task(1)), (now + 2, async_task(2))).is_ok());
    }

    async fn pending_task(number: u8) {
        println!("Starting infinite task {}", number);
        std::thread::sleep(std::time::Duration::from_secs(1));
        let () = std::future::pending().await;
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_missing_deadline() {
        assert!(spawn!((StdTimer.now() + 2, pending_task(1))).is_err());
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
        executor.spawn(Task::new(1, DelayStrategy::ReturnError, long_task(10)));
        assert!(executor.run().is_err());

        let mut executor = Executor::new();
        executor.spawn(Task::new(1, DelayStrategy::ContinueRunning, long_task(3)));
        assert!(executor.run().is_ok());
    }
}
