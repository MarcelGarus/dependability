# retry

Proof of concept for a Rust [procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html) to rerun a function if it fails. 
Supports functions returning `Result` or `Option`.

## Usage

```rust
#[retry(3)]
fn might_fail() -> Result<(), SomeError> {
    /* ... */
}
```

This example will run up to four attempts of `might_fail`. You lose the information of possible errors.
Instead, a function with the `#[retry]` macro will return that it has exceeded its number of retries.

## Warning

This is just a proof of concept. Procedural macros are dangerous and I am no expert, so there are
probably a lot of cases in which this crate will generate wrong code. Proceed at your own risk if
you use it!
