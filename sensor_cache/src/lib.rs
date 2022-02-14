#![no_std]

use core::cell::RefCell;

use alloc::{boxed::Box, vec::Vec};

extern crate alloc;

pub trait SensorCache<T, E> {
    fn read(&self) -> SensorValue<T, E>;
    fn expected_value(&self) -> T;
}

#[derive(Debug)]
pub enum SensorValue<T, E> {
    Plausible(T),
    Implausible(T),
    Error(T, E),
}

pub struct SmoothedSensor<T, E> {
    cache: RefCell<Vec<T>>,
    read_fn: Box<dyn Fn() -> Result<T, E>>,
    epsilon: T,
}

impl<T, E> SmoothedSensor<T, E> {
    pub fn new(read_fn: Box<dyn Fn() -> Result<T, E>>, epsilon: T) -> Self {
        Self {
            cache: RefCell::new(Vec::new()),
            read_fn,
            epsilon,
        }
    }
}

impl<E> SensorCache<f64, E> for SmoothedSensor<f64, E> {
    fn read(&self) -> SensorValue<f64, E> {
        match (self.read_fn)() {
            Ok(value) => {
                if self.cache.borrow().is_empty()
                    || libm::fabs(value - self.expected_value()) <= self.epsilon
                {
                    self.cache.borrow_mut().push(value);
                    SensorValue::Plausible(value)
                } else {
                    SensorValue::Implausible(value)
                }
            }
            Err(error) => SensorValue::Error(self.expected_value(), error),
        }
    }

    fn expected_value(&self) -> f64 {
        //let n_samples = self.cache.borrow().len() as f64;
        //self.cache.borrow().iter().sum::<f64>() / n_samples
        *self.cache.borrow().last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use core::convert::Infallible;
    use std::dbg;

    use alloc::boxed::Box;

    use crate::{SensorCache, SmoothedSensor};

    #[test]
    fn test_plausible() {
        let sensor = || -> Result<f64, Infallible> {
            static mut VALUE: f64 = 0.0;
            unsafe { VALUE += 0.1 };
            unsafe { Ok(VALUE) }
        };

        let smoothed = SmoothedSensor::new(Box::new(sensor), 0.2);
        for i in 1..10 {
            match smoothed.read() {
                crate::SensorValue::Plausible(v) => {
                    assert!(libm::fabs(v - (i as f64) / 10.0) < f64::EPSILON)
                }
                crate::SensorValue::Implausible(_) => unreachable!(),
                crate::SensorValue::Error(_, _) => unreachable!(),
            }
        }
    }

    #[test]
    fn test_implausible() {
        let sensor = || -> Result<f64, Infallible> {
            static mut VALUE: u64 = 0;
            if unsafe { VALUE != 5 } {
                unsafe { VALUE += 1 };
            } else {
                unsafe { VALUE += 11 };
            }
            unsafe { Ok((VALUE as f64) / 10.0) }
        };

        let smoothed = SmoothedSensor::new(Box::new(sensor), 0.2);
        for i in 1..10 {
            match smoothed.read() {
                crate::SensorValue::Plausible(v) => {
                    assert!(libm::fabs(v - (i as f64) / 10.0) < f64::EPSILON)
                }
                crate::SensorValue::Implausible(v) => {
                    assert!(libm::fabs((v - 1.0) - (i as f64) / 10.0) < f64::EPSILON)
                }
                crate::SensorValue::Error(_, _) => unreachable!(),
            }
        }
    }
}
