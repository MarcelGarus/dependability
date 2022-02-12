#![no_std]
#![feature(allocator_api)]

#[cfg(feature = "runtime")]
pub use dependability_runtime as runtime;

#[cfg(feature = "retry")]
pub use dependability_retry as retry;

#[cfg(feature = "alloc")]
pub use dependability_alloc as alloc;

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    extern crate std;
    use std::println;

    use dependability_alloc::collections;
    use dependability_runtime::{
        task::{executor::Executor, noop::noop, DelayStrategy, Task},
        time::{StdTimer, Timer},
    };
    use std::alloc::Global;

    async fn complex_task(id: usize) {
        println!("Task {} here!", id);
        let mut v: collections::Vec<_, 64, u8> = collections::Vec::new(Global);
        for i in 0..8 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            v.push(i).await;
            noop().await;
        }
        for i in 0..8 {
            println!("{}", v.get(i));
        }
    }

    #[test]
    fn test_state_machine() {
        let mut executor = Executor::new();
        let now = StdTimer.now();

        executor.spawn(Task::new(
            now + 16,
            DelayStrategy::ReturnError,
            complex_task(0),
        ));
        //executor.spawn(Task::new(
        //    now + 8,
        //    DelayStrategy::ReturnError,
        //    complex_task(1),
        //));
        //assert!(executor.run().is_err());
        assert!(executor.run().is_ok());
    }
}
