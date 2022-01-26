use core::time::Duration;

/// A imestamp in seconds.
pub type Timestamp = u64;

/// A timer to be implemented for different platforms
pub trait Timer {
    /// The current timestamp. This does not have to be absolute
    /// but could e.g. be the time since the microcontroller was started
    fn now(&self) -> Timestamp;

    /// Suspend the current execution context for the given [`Duration`]
    fn delay(&self, duration: Duration);

    /// Calculate how much time has passed since the given timestamp.
    fn elapsed_since(&self, since: Timestamp) -> Timestamp {
        self.now() - since
    }
}

pub trait IntoDuration {
    fn millis(self) -> Duration;
}

impl IntoDuration for u64 {
    fn millis(self) -> Duration {
        Duration::from_millis(self)
    }
}

#[cfg(feature = "std")]
#[derive(Default)]
pub struct StdTimer;

#[cfg(feature = "std")]
impl Timer for StdTimer {
    fn now(&self) -> Timestamp {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn delay(&self, duration: Duration) {
        std::thread::sleep(duration);
    }
}
