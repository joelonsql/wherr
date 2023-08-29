# `wherr` Crate

[![version](https://img.shields.io/crates/v/wherr.svg)](https://crates.io/crates/wherr)
[![documentation](https://docs.rs/wherr/badge.svg)](https://docs.rs/wherr)
[![License](https://img.shields.io/crates/l/wherr)](./LICENSE)

[Discuss `wherr` on Hacker News](https://news.ycombinator.com/item?id=37232229)

Enhance Rust's `?` operator by appending file and line number details to errors, simplifying the debugging process.

## Features

- 📁 Appends file and line number to errors propagated with `?`.
- 🧩 Integrates seamlessly with Rust's existing error handling.
- 📍 Helps locate and fix errors faster by pinpointing their origin.
- 🚀 Supports `anyhow` error handling through the `anyhow` feature flag.

## Why `wherr`?

When using the `?` operator in Rust, errors propagated don't usually provide file and line number details about **where** the error occurs. Especially in nested function calls with multiple potential error points, this absence of detailed info can be a debugging headache.

`wherr` remedies this, giving you precise location data for your errors.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
wherr = "0.1"
```

For `anyhow` support:

```toml
[dependencies]
wherr = { version = "0.1", features = ["anyhow"] }
```

## Quick Start

By simply annotating your functions with `#[wherr]`, errors propagated using `?` will also include the file and line number.

1. **Standard Usage**: Annotate functions with `#[wherr]`.

```rust
use wherr::wherr;

#[wherr]
fn concat_files(path1: &str, path2: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut content1 = std::fs::read_to_string(path1)?;
    let content2 = std::fs::read_to_string(path2)?;

    content1.push_str(&content2);
    Ok(content1)
}
```

Run the provided example:

```sh
cargo run --example with_wherr
```

This will highlight the difference `wherr` makes. Your error will now indicate exactly where the issue arises:

```
error at wherr/examples/with_wherr.rs:5
```

2. **With `anyhow`**:

Ensure you've added the `anyhow` feature as shown in the installation section. Then:

```rust
use anyhow::{Context, Result};
use wherr::wherr;

#[wherr]
fn concat_files(path1: &str, path2: &str) -> Result<String> {
    let mut content1 = std::fs::read_to_string(path1).with_context(|| format!("Failed to read {}", path1))?;
    let content2 = std::fs::read_to_string(path2).with_context(|| format!("Failed to read {}", path2))?;

    content1.push_str(&content2);
    Ok(content1)
}
```

Run the provided example:

```sh
cargo run --features=anyhow --example anyhow
```

This will highlight the difference `wherr` makes. Your error will now indicate exactly where the issue arises:

```
error at wherr/examples/anyhow.rs:6
```

## Behind the Scenes

The `#[wherr]` notation is a [proc_macro_attribute](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros), which allows for code transformations at compile time.

When an error is propagated using the `?` operator inside a `#[wherr]` annotated function, the macro captures the file and line number where the error occurred.

Essentially, the function:

```rust
#[wherr]
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    ...
    let i1 = i64::from_str_radix(s1, radix)?;
    ...
}
```

... is transformed to:

```rust
fn add(s1: &str, s2: &str) -> Result<i64, Box<dyn std::error::Error>> {
    ...
    let i1 = wherr::wherrapper(i64::from_str_radix(s1, radix), file!(), line!())?;
    ...
}
```

The magic happens inside `wherrapper`. It checks the given result, and if it's an error, wraps it with file and line details.

## Usage & Examples

Dive deeper into the problem `wherr` solves and the Rust concepts involved with our [detailed examples](./examples/README.md).

## Note on Crate Organization

`wherr` is divided into two separate crates because of Rust's restriction against mixing normal functions and procedural macros in the same crate:

1. [`wherr`](https://crates.io/crates/wherr):
   This is the main library that offers the enhanced functionality for error
   handling in Rust.
2. [`wherr-macro`](https://crates.io/crates/wherr-macro):
   Contains the procedural macros specifically designed for the `wherr` crate.

## Contributing

If you're interested in contributing to `wherr`, please follow standard
Rust community guidelines and submit a PR on our repository.

## License

This project is dual licensed under [MIT License](./LICENSE-MIT) and [Apache License](./LICENSE-APACHE).
