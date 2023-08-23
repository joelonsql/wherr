# `wherr` Crate Documentation

`wherr` extends Rust's `?` operator to append file and line number details to
errors, aiding in debugging.

`wherr` is composed of two separate crates, due to Rust's limitation that
prohibits mixing normal functions with procedural macros in a single crate:

1. [`wherr`](https://crates.io/crates/wherr):
   This is the main library that offers the enhanced functionality for error
   handling in Rust.
2. [`wherr-macro`](https://crates.io/crates/wherr-macro):
   Contains the procedural macros specifically designed for the `wherr` crate.

## Table of Contents

- [Quick Start](#quick-start)
- [What's the problem?](#whats-the-problem)
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

Note, that the Error type needs to be `Box<dyn std::error::Error>`.

```rust
 1 | use wherr::wherr;
 2 |
 3 | #[wherr]
 4 | fn concat_files(path1: &str, path2: &str) -> Result<String, Box<dyn std::error::Error>> {
 5 |     let mut content1 = std::fs::read_to_string(path1)?;
 6 |     let content2 = std::fs::read_to_string(path2)?;
 7 |
 8 |     content1.push_str(&content2);
 9 |     Ok(content1)
10 | }
11 |
12 | fn main() {
13 |     let content = concat_files("file1.txt", "file2.txt").expect("Failed to concatenate the files");
14 |     println!("Concatenated content:\n{}", content);
15 | }
```

```sh
% cargo run --example with_wherr
```
```
% cargo run --example with_wherr
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/examples/with_wherr`
thread 'main' panicked at 'Failed to concatenate the files: Os { code: 2, kind: NotFound, message: "No such file or directory" }
error at wherr/examples/with_wherr.rs:5', wherr/examples/with_wherr.rs:13:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Notice the text added by `wherr` in the output: **error at wherr/examples/with_wherr.rs:5**

This tells us where the error originated from:

```rust
 5 |     let mut content1 = std::fs::read_to_string(path1)?;
```

## What's the problem?

The problem is that when simply using `?` to propagate errors, you don't get file and line information on **where** the error occurs.

To demonstrate the problem, let's run the same example as above, but without `wherr`.

```rust
 1 | fn concat_files(path1: &str, path2: &str) -> Result<String, std::io::Error> {
 2 |     let mut content1 = std::fs::read_to_string(path1)?;
 3 |     let content2 = std::fs::read_to_string(path2)?;
 4 |
 5 |     content1.push_str(&content2);
 6 |     Ok(content1)
 7 | }
 8 |
 9 | fn main() {
10 |     let content = concat_files("file1.txt", "file2.txt").expect("Failed to concatenate the files");
11 |     println!("Concatenated content:\n{}", content);
12 | }
```

```sh
% cargo run --example without_wherr
```
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/without_wherr`
thread 'main' panicked at 'Failed to concatenate the files: Os { code: 2, kind: NotFound, message: "No such file or directory" }', wherr/examples/without_wherr.rs:10:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

As you can see, the only file and line information we get is `wherr/examples/without_wherr.rs:10:58`, which is this line:

```rust
10 |     let content = concat_files("file1.txt", "file2.txt").expect("Failed to concatenate the files");
```

Unfortunately, if we suspect we have a bug somewhere in `concat_files()`,
we might need to know what specific line that causes a certain error.

This might be tricky, especially when you have nested layers of functions,
where the same error kind might occurr at multiple different places.

In our example, there are two calls to `read_to_string()`. Any one of them
could have caused the `NotFound` kind of error.

Running with `RUST_BACKTRACE=1` doesn't help:

```sh
% RUST_BACKTRACE=1 cargo run --example without_wherr
```
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/examples/without_wherr`
thread 'main' panicked at 'Failed to concatenate the files: Os { code: 2, kind: NotFound, message: "No such file or directory" }', wherr/examples/without_wherr.rs:10:58
stack backtrace:
   0: rust_begin_unwind
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:579:5
   1: core::panicking::panic_fmt
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:64:14
   2: core::result::unwrap_failed
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/result.rs:1750:5
   3: core::result::Result<T,E>::expect
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/result.rs:1047:23
   4: without_wherr::main
             at ./wherr/examples/without_wherr.rs:10:19
   5: core::ops::function::FnOnce::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Still only `without_wherr.rs:10`.

Not even `RUST_BACKTRACE=full` helps:

```sh
% RUST_BACKTRACE=full cargo run --example without_wherr
```
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/examples/without_wherr`
thread 'main' panicked at 'Failed to concatenate the files: Os { code: 2, kind: NotFound, message: "No such file or directory" }', wherr/examples/without_wherr.rs:10:58
stack backtrace:
   0:        0x10266beec - std::backtrace_rs::backtrace::libunwind::trace::h2000fb4d08dbcc59
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:        0x10266beec - std::backtrace_rs::backtrace::trace_unsynchronized::h2b5e61495350674d
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:        0x10266beec - std::sys_common::backtrace::_print_fmt::h05f5bfbdb3415936
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:65:5
   3:        0x10266beec - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h105074e3d85f800b
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:44:22
   4:        0x10267d820 - core::fmt::write::h34766cf8fff7af1e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/fmt/mod.rs:1232:17
   5:        0x102669f60 - std::io::Write::write_fmt::hd64c4cf6e7adea59
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/io/mod.rs:1684:15
   6:        0x10266bd00 - std::sys_common::backtrace::_print::hd92783a665d3ebfb
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:47:5
   7:        0x10266bd00 - std::sys_common::backtrace::print::h2a6828a537036cf9
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:34:9
   8:        0x10266d2ac - std::panicking::default_hook::{{closure}}::h4e82ce6ccef941b2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:271:22
   9:        0x10266d004 - std::panicking::default_hook::h29f62f8795c5cb00
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:290:9
  10:        0x10266d7bc - std::panicking::rust_panic_with_hook::h19862cbd0fbda7ba
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:692:13
  11:        0x10266d6f0 - std::panicking::begin_panic_handler::{{closure}}::h3f3626935e1669fe
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:583:13
  12:        0x10266c30c - std::sys_common::backtrace::__rust_end_short_backtrace::h5054ef52bd507d0a
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:150:18
  13:        0x10266d44c - rust_begin_unwind
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:579:5
  14:        0x102684590 - core::panicking::panic_fmt::h7e47e10600a90221
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:64:14
  15:        0x102684838 - core::result::unwrap_failed::h6a1757e313e2d291
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/result.rs:1750:5
  16:        0x1026519b0 - core::result::Result<T,E>::expect::h3bb438a6543db6fe
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/result.rs:1047:23
  17:        0x102653950 - without_wherr::main::hea9e71193dce0291
                               at /Users/joel/src/wherr/wherr/examples/without_wherr.rs:10:19
  18:        0x102653218 - core::ops::function::FnOnce::call_once::ha7443c5406a2bc4c
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
  19:        0x1026518b4 - std::sys_common::backtrace::__rust_begin_short_backtrace::ha104f92716531d05
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/sys_common/backtrace.rs:134:18
  20:        0x102650b40 - std::rt::lang_start::{{closure}}::h4d7ab56f8229bc15
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:166:18
  21:        0x102667604 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hf2f6b444963da11f
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:287:13
  22:        0x102667604 - std::panicking::try::do_call::h9152231fddd58858
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  23:        0x102667604 - std::panicking::try::hcc27eab3b8ee3cb1
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  24:        0x102667604 - std::panic::catch_unwind::hca546a4311ab9871
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  25:        0x102667604 - std::rt::lang_start_internal::{{closure}}::h4e65aa71fe685c85
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:48
  26:        0x102667604 - std::panicking::try::do_call::h61aea55fbdf97fc2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:487:40
  27:        0x102667604 - std::panicking::try::hcfc3b62fb8f6215e
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:451:19
  28:        0x102667604 - std::panic::catch_unwind::h61a201e98b56a743
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panic.rs:140:14
  29:        0x102667604 - std::rt::lang_start_internal::h91996717d3eb1d2a
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:148:20
  30:        0x102650b0c - std::rt::lang_start::ha5cfaf50836838f2
                               at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/rt.rs:165:17
  31:        0x102653a08 - _main
```

Still only `without_wherr.rs:10`.

## How it works

The #[wherr] notation is a [proc_macro_attribute](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros),
a powerful tool in Rust that allows for custom transformations of the code at
compile time.

When an error is propagated with the `?` operator in a function decorated with
`#[wherr]`, it captures the file and line number where the error occurred.
This provides developers with a more informative error, aiding in precise
debugging.

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

Now, you may be wondering: How does this `wherrapper` function make all
this happen?

The `wherrapper` function receives the original `Result`, the file, and the line
number. If the `Result` is an `Ok`, it's simply returned unchanged.
For an `Err`, however, things are a bit more intricate.
If the error hasn't already been wrapped in a Wherr type (meaning it doesn't yet
have the file and line details), the function wraps it in a new `Wherr` type
that contains the original error, as well as the file and line information.
If the error is already a `Wherr` type, it's left unchanged, ensuring that we
retain the original location of the error.

Here's a brief look at the `wherrapper` function:

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
a function annotated with `#[wherr]` will the precise location of where the
error occurred in the code.
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
x = Err(ParseIntError { kind: InvalidDigit }
error at wherr/examples/basic_with_wherr.rs:10)
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
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
error at wherr/examples/unwrap_with_wherr.rs:10', wherr/examples/unwrap_with_wherr.rs:15:40
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

Here is the diff:

```diff
-thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value:
-ParseIntError { kind: InvalidDigit }', wherr/examples/unwrap_without_wherr.rs:12:40
+thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
+error at wherr/examples/unwrap_with_wherr.rs:10', wherr/examples/unwrap_with_wherr.rs:15:40
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

This internal utility function is used by the procedural macro to wrap errors
with file and line information.

```rust
pub fn wherrapper<T, E>(
    result: Result<T, E>,
    file: &'static str,
    line: u32,
) -> Result<T, Box<dyn std::error::Error>>
```

### `wherr` procedural macro

A procedural macro that auto-wraps errors (using the `?` operator) inside
a function with file and line number details.

```rust
#[wherr]
fn some_function() -> Result<(), Box<dyn std::error::Error>> { /* ... */ }
```

## Contributing

If you're interested in contributing to `wherr`, please follow standard
Rust community guidelines and submit a PR on our repository.

## License

Dual license. See the `LICENSE-MIT` and `LICENSE-APACHE` files.
