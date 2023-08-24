# `wherr` Examples

In this guide, we'll delve deeper into using the `wherr` crate with practical examples. These examples help demonstrate the value `wherr` adds to error handling in Rust by appending file and line number details to errors.

## Basic Usage

### Without `wherr`

In our basic example, we try to add two numbers represented as strings:

`examples/without_wherr.rs`:

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

Executing the code:

```sh
% cargo run --example without_wherr
```

Yields:

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/examples/without_wherr`
thread 'main' panicked at 'Failed to concatenate the files: Os { code: 2, kind: NotFound, message: "No such file or directory" }', wherr/examples/without_wherr.rs:10:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

As observed, the error does not provide information on where the *error* occurred, only where the *panic* occurred:

```rust
10 |     let content = concat_files("file1.txt", "file2.txt").expect("Failed to concatenate the files");
```

Unfortunately, if we suspect we have a bug somewhere in `concat_files()`, we might need to know what specific line that causes a certain error.

This might be tricky, especially when you have nested layers of functions, where the same error kind might occurr at multiple different places.

In our example, there are two calls to `read_to_string()`. Any one of them could have caused the `NotFound` kind of error.

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

### With `wherr`

Let's integrate `wherr` into the mix:

`examples/with_wherr.rs`:

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

Executing the code:

```sh
% cargo run --example with_wherr
```

Yields:

```
% cargo run --example with_wherr
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/examples/with_wherr`
thread 'main' panicked at 'Failed to concatenate the files: Os { code: 2, kind: NotFound, message: "No such file or directory" }
error at wherr/examples/with_wherr.rs:5', wherr/examples/with_wherr.rs:13:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Notice the appended file and line number, **error at wherr/examples/with_wherr.rs:5**, pinpointing exactly where the error arose:

The real power of `wherr` manifests in complex codebases with multiple nested function calls, where pinpointing errors can be challenging. While the basic example provides a glimpse into its utility, using `wherr` in larger projects can be invaluable for precise debugging.
