# `wherr` Crate Documentation

`wherr` extends Rust's `?` operator to append file and line number details to errors, aiding in debugging.

`wherr` is composed of two separate crates, due to Rust's limitation that prohibits mixing normal functions with procedural macros in a single crate:

1. [`wherr`](https://crates.io/crates/wherr): This is the main library that offers the enhanced functionality for error handling in Rust.
2. [`wherr-macro`](https://crates.io/crates/wherr-macro): Contains the procedural macros specifically designed for the `wherr` crate.

## Table of Contents

- [Quick Start](#quick-start)
- [How it works](#how-it-works)
- [Usage](#usage)
  - [Without `#[wherr]`](#without-wherr)
  - [With `#[wherr]`](#with-wherr)
- [API Reference](#api-reference)
  - [`Wherr`](#wherr)
  - [`wherrapper`](#wherrapper)
  - [`wherr` procedural macro](#wherr-procedural-macro)
- [Contributing](#contributing)
- [License](#license)

## Quick Start

Add the `wherr` crate to your `Cargo.toml`:

```toml
[dependencies]
wherr = "0.1"
```

Now, by simply annotating your functions with `#[wherr]`, any error propagated
using `?` will also include the file and line number.

```rust
use wherr::wherr;

#[wherr]
fn some_function() -> Result<(), Box<dyn std::error::Error>> {
    /* ... */
    some_operation()?;
    /* ... */
}
```

## How it works

The #[wherr] notation is a [proc_macro_attribute](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros),
a powerful tool in Rust that allows for custom transformations of the code at compile time.

The purpose of this attribute is to enhance the error handling in Rust.
When you use `?` to propagate errors, by default, you only get the error message.
With `#[wherr]`, the idea is to provide richer context: the exact file and line number where the error occurred.

For instance, consider this function:

```rust
#[wherr]
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = i64::from_str_radix(s1, radix)?;
    let i2 = i64::from_str_radix(s2, radix)?;
    Ok(i1 + i2)
}
```

Under the hood, the `#[wherr]` macro transforms this function to:

```rust
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = wherr::wherrapper(i64::from_str_radix(s1, radix), file!(), line!())?;
    let i2 = wherr::wherrapper(i64::from_str_radix(s2, radix), file!(), line!())?;
    Ok(i1 + i2)
}
```

Now, you may be wondering: How does this `wherrapper` function make all this happen?

The `wherrapper` function, defined below, takes in the original `Result`,
the file, and the line number. If the `Result` is an `Ok`, it passes through
unchanged. If it's an `Err`, it wraps the error into a new `Wherr` type which
contains the original error alongside the file and line number:

```rust
pub fn wherrapper<T, E>(
    result: Result<T, E>,
    file: &'static str,
    line: u32,
) -> Result<T, Box<dyn std::error::Error>>
where
    E: Into<Box<dyn std::error::Error>>,
{
    match result {
        Ok(val) => Ok(val),
        Err(err) => {
            let boxed_err: Box<dyn std::error::Error> = err.into();
            if boxed_err.is::<Wherr>() {
                Err(boxed_err)
            } else {
                Err(Box::new(Wherr::new(boxed_err, file, line)))
            }
        }
    }
}
```

Through this mechanism, any error returned (propagated using `?`) from
a function annotated with `#[wherr]` will provide not just the error message
but also the precise location of where the error occurred in the code.
This offers developers better insight during debugging sessions.

## Usage

To understand the benefits of the `wherr` crate, let's first observe the problem
it aims to solve.

If you already know Rust, feel free to skip ahead to the [Without wherr](#without-wherr)
section, since the following will breifly explain the Rust concepts of
the [`Result<T, E>`](https://doc.rust-lang.org/std/result/) type,
and the [`?` operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator).

In these examples, we utilize the `i64::from_str_radix(s1, radix)` function
from the standard library, which has the signature:

```rust
pub fn from_str_radix(src: &str, radix: u32) -> Result<i64, ParseIntError>
```

This function aims to convert a string slice, representing a number in a
specified base, into an integer. It returns a `Result` type —- an *enum* in Rust.
The `Result` enum comprises two *variants*: `Ok` and `Err`. Notably, in Rust,
these *variants* can encapsulate data. For `Result<i64, ParseIntError>`,
the `Ok` variant wraps an `i64` value, whereas the `Err` variant encapsulates
a `ParseIntError` value.

In the line:

```rust
let i = i64::from_str_radix(s, radix)?;
```

We see the usage of the `?` [operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator).
This operator is used in Rust for a concise error handling.

When placed after a function that returns a `Result`, it does two things:

1. If the function returns an `Ok` variant, the `?` operator extracts the value
   inside `Ok` and assigns it to the variable (in this case `i`).

2. If the function returns an `Err` variant, the `?` operator immediately
   returns this error, effectively short-circuiting any further operations
   in the function.

Let's make an experiment demonstrating both variants.

### Without `#[wherr]`:

`examples/basic_without_wherr.rs`:
```rust
// Function to add two numbers represented as strings.
// Returns a Result with the sum within an `Ok` variant if successful,
// or an `Err` variant if there's an error.
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = i64::from_str_radix(s1, radix)?;
    let i2 = i64::from_str_radix(s2, radix)?;
    Ok(i1 + i2)
}

fn main() {
    let x = add("123", "not a number");
    println!("x = {:?}", x);
}
```

```sh
cargo run --example basic_without_wherr
```
```
x = Err(ParseIntError { kind: InvalidDigit })
```

Note that the `Err` lacks file or line details.

Using `.unwrap()` extracts the `Ok` value or panics on error. While the panic
shows the file and line number, it only indicates the `.unwrap()` location,
not the error's origin. Even with `RUST_BACKTRACE=1` or `RUST_BACKTRACE=full`,
the error's origin remains elusive. As it's a returned value, possibly
passed through many functions, without embedded file or line info, retrieval
is impossible.

`examples/unwrap_without_wherr.rs`:
```rust
// The add() function is as previously defined and is omitted here for clarity.

fn main() {
    let x = add("123", "not a number").unwrap();
    println!("x = {:?}", x);
}
```

```sh
cargo run --example unwrap_without_wherr
```
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
ParseIntError { kind: InvalidDigit }', wherr/examples/unwrap_without_wherr.rs:12:40
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The line **wherr/examples/unwrap_without_wherr.rs:12** corresponds to:
```rust
    let x = add("123", "not a number").unwrap();
```

Now, let's have a look at the same examples, but this time with `#[wherr]`
enabled.

### With `#[wherr]`:

By adding `#[wherr]` macro to the function, errors will automatically
be wrapped in a `Wherr` struct with a `file` and `line` field
telling us where the error happened. The original error is preserved and
accessible via the `inner` field.

`examples/basic_with_wherr.rs`:
```rust
use wherr::wherr;

// Function to add two numbers represented as strings.
// Returns a Result with the sum within an `Ok` variant if successful,
// or an `Err` variant if there's an error.
#[wherr]
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = i64::from_str_radix(s1, radix)?;
    let i2 = i64::from_str_radix(s2, radix)?;
    Ok(i1 + i2)
}

fn main() {
    let x = add("123", "not a number");
    println!("x = {:?}", x);
}
```

```sh
cargo run --example basic_with_wherr
```
```
x = Err(Error at wherr/examples/basic_with_wherr.rs:10. Original error: ParseIntError { kind: InvalidDigit })
```

The line **wherr/examples/basic_with_wherr.rs:10** corresponds to:
```rust
    let i2 = i64::from_str_radix(s2, radix)?;
```

`examples/unwrap_with_wherr.rs`:
```rust
use wherr::wherr;

// Function to add two numbers represented as strings.
// Returns a Result with the sum within an `Ok` variant if successful,
// or an `Err` variant if there's an error.
#[wherr]
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let radix = 10;
    let i1 = i64::from_str_radix(s1, radix)?;
    let i2 = i64::from_str_radix(s2, radix)?;
    Ok(i1 + i2)
}

fn main() {
    let x = add("123", "not a number").unwrap();
    println!("x = {:?}", x);
}
```

```sh
cargo run --example unwrap_with_wherr
```
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
Error at wherr/examples/unwrap_with_wherr.rs:10.
Original error: ParseIntError { kind: InvalidDigit }', wherr/examples/unwrap_with_wherr.rs:15:40
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The line **wherr/examples/unwrap_with_wherr.rs:10** corresponds to:
```rust
    let i2 = i64::from_str_radix(s2, radix)?;
```

And the line **wherr/examples/unwrap_with_wherr.rs:15** corresponds to:
```rust
    let x = add("123", "not a number").unwrap();
```

Here is the diff (additional newlines added for clarity):

```diff
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
-ParseIntError { kind: InvalidDigit }', wherr/examples/unwrap_without_wherr.rs:12:40
+Error at wherr/examples/unwrap_with_wherr.rs:10.
+Original error: ParseIntError { kind: InvalidDigit }', wherr/examples/unwrap_with_wherr.rs:15:40
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The `file` and `line` info can also be extracted from the `Wherr` struct,
that wraps the original `Err`:

```rust
    match add("123", "not a number") {
        Ok(sum) => {
            println!("sum = {}", sum);
        }
        Err(e) => {
            if let Some(wherr) = e.downcast_ref::<wherr::Wherr>() {
                println!(
                    "Error at file: '{}', line: {}. Original error: {}",
                    wherr.file, wherr.line, wherr.inner
                );
            } else {
                println!("Unexpected error: {}", e);
            }
        }
    }
```

It also works through multiple nested layers of `?`. The `Err` is only wrapped
inside a `Wherr` once, and then propagated unchanged upwards.

## API Reference

### `Wherr`

Represents an error that includes file and line number information.

```rust
pub struct Wherr {
    inner: Box<dyn std::error::Error>,
    file: &'static str,
    line: u32,
}
```

Methods:

- `new(err: Box<dyn std::error::Error>, file: &'static str, line: u32) -> Self`: Creates a new `Wherr` error that wraps another error, providing additional context.

### `wherrapper`

This internal utility function is used by the procedural macro to wrap errors with file and line information.

```rust
pub fn wherrapper<T, E>(
    result: Result<T, E>,
    file: &'static str,
    line: u32,
) -> Result<T, Box<dyn std::error::Error>>
```

### `wherr` procedural macro

A procedural macro that auto-wraps errors (using the `?` operator) inside a function with file and line number details.

```rust
#[wherr]
fn some_function() -> Result<(), Box<dyn std::error::Error>> { /* ... */ }
```

## Contributing

If you're interested in contributing to `wherr`, please follow standard Rust community guidelines and submit a PR on our repository.

## License

Please refer to the `LICENSE` file in the root directory of the crate for license details.
