use retry::retry;

static mut X: u32 = 0;

// Functions with arguments are also supported!
#[retry(2)]
fn min(i: f32, j: f32) -> Option<f32> {
    if i.is_nan() || j.is_nan() {
        None
    } else {
        Some(i.min(j))
    }
}

// If your function returns an Option<T>,
// the generated function returns a Result<T, &'static str>
#[retry(3)]
fn returns_option() -> Option<u32> {
    None
}

// Result<T, E> is mapped to Result<T, &'static str>
#[retry(2)]
fn might_fail() -> Result<u32, &'static str> {
    match unsafe { X } {
        0..=1 => {
            unsafe { X += 1 };
            Err("Error")
        }
        _ => Ok(unsafe { X }),
    }
}

fn main() {
    let f = might_fail().unwrap();
    dbg!(f);
}
