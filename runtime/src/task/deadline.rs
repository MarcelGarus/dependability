use core::ops::Add;

use crate::time::Timestamp;

#[derive(Clone, Copy)]
pub enum Deadline {
    Finite(Timestamp),
    Infinite,
}

impl From<Timestamp> for Deadline {
    fn from(t: Timestamp) -> Self {
        Self::Finite(t)
    }
}

impl PartialEq for Deadline {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Finite(l0), Self::Finite(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Eq for Deadline {}

impl PartialOrd for Deadline {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        use core::cmp::Ordering::{Greater, Less};
        match (self, other) {
            (Deadline::Finite(left), Deadline::Finite(right)) => left.partial_cmp(right),
            (Deadline::Finite(_), Deadline::Infinite) => Some(Less),
            (Deadline::Infinite, Deadline::Finite(_)) => Some(Greater),
            (Deadline::Infinite, Deadline::Infinite) => None,
        }
    }
}

impl Ord for Deadline {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        use core::cmp::Ordering::{Equal, Greater, Less};
        match (self, other) {
            (Deadline::Finite(left), Deadline::Finite(right)) => left.cmp(right),
            (Deadline::Finite(_), Deadline::Infinite) => Less,
            (Deadline::Infinite, Deadline::Finite(_)) => Greater,
            (Deadline::Infinite, Deadline::Infinite) => Equal,
        }
    }
}

impl Add<u64> for Deadline {
    type Output = Deadline;

    fn add(self, rhs: u64) -> Self::Output {
        match self {
            Deadline::Finite(ts) => Deadline::Finite(ts + rhs),
            Deadline::Infinite => Deadline::Infinite,
        }
    }
}
