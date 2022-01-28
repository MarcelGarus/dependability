#![no_std]

pub mod proc_macro {
    pub use dependability_retry_proc_macro::retry;
}

#[derive(Debug, PartialEq)]
pub struct RetryError;

#[macro_export]
macro_rules! retry {
    ($fn_call:expr, $retries:tt) => {{
        let mut tries = 0;
        loop {
            match ($fn_call) {
                Ok(v) => break Ok(v),
                Err(_) => {
                    if tries < $retries {
                        tries += 1;
                    } else {
                        break Err(RetryError);
                    }
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use core::convert::Infallible;

    use crate::RetryError;

    fn f() -> Result<i32, Infallible> {
        Ok(42)
    }

    fn g() -> Result<i32, &'static str> {
        Err("nope")
    }

    #[test]
    fn succeeds() {
        assert_eq!(retry!(f(), 3), Ok(42));
    }

    #[test]
    fn fails() {
        assert_eq!(retry!(g(), 3), Err(RetryError));
    }
}
