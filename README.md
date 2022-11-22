# human-panic

[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

This is a Jetty Labs fork of the original `human-panic` crate. We mostly have just made the project more formal and flexible and it might be a good idea to upstream changes as we make them.

Panic messages for humans. Handles panics by calling
[`std::panic::set_hook`](https://doc.rust-lang.org/std/panic/fn.set_hook.html) to make errors nice for humans.

-   [Documentation][8]
-   [Crates.io][2]

## Why?

When you're building a CLI, polish is super important. Even though Rust is
pretty great at safety, it's not unheard of to access the wrong index in a
vector or have an assert fail somewhere.

When an error eventually occurs, you probably will want to know about it. So
instead of just providing an error message on the command line, we can create a
call to action for people to submit a report.

This should empower people to engage in communication, lowering the chances
people might get frustrated. And making it easier to figure out what might be
causing bugs.

### Default Output

```txt
thread 'main' panicked at 'oops', examples/main.rs:2:3
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

### Human-Panic Output

```txt
jetty_cli had a problem and crashed. To help us diagnose the problem you can send us a crash report.

We have generated a report file at "/var/folders/wp/msj9z_4d7jj1jlq8d771cxzr0000gn/T/report-b5e4e45d-026a-4978-9f06-33f075b53dcd.toml". Submit an issue or email with the subject of "jetty_cli Crash Report" and include the report as an attachment.

Thank you!
```

The error dump file generated by `human-panic` contains the following fields.

```toml
name = 'single-panic-test'
operating_system = 'unix:Unknown'
crate_version = '0.1.0'
explanation = '''
Cause: OMG EVERYTHING IS ON FIRE!!!. Panic occurred in file 'tests/single-panic/src/main.rs' at line 8
'''
method = 'Panic'
backtrace = '''
stack backtrace:
   0:     0x55fa0ed4c1b4 - backtrace::backtrace::libunwind::trace::h69e50feca54bfb84
                        at /home/spacekookie/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.6/src/backtrace/libunwind.rs:53
                         - backtrace::backtrace::trace::h42967341e0b01ccc
                        at /home/spacekookie/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.6/src/backtrace/mod.rs:42

    # ...

   8:     0x55fa0ebaac8d - single_panic_test::main::h56a3d326bcecfc36
                        at tests/single-panic/src/main.rs:8
   9:     0x55fa0ebaae91 - std::rt::lang_start::{{closure}}::h09d28d8540038bf8
                        at /checkout/src/libstd/rt.rs:74
  10:     0x55fa0ed732f7 - std::rt::lang_start_internal::{{closure}}::h2e4baf0a27c956a3
                        at libstd/rt.rs:59
                         - std::panicking::try::do_call::h73f98ed0647c7274
                        at libstd/panicking.rs:305
  11:     0x55fa0ed8551e - __rust_maybe_catch_panic
                        at libpanic_unwind/lib.rs:101
  12:     0x55fa0ed6f7f5 - std::panicking::try::h18fbb145180d4cd9
                        at libstd/panicking.rs:284
                         - std::panic::catch_unwind::hc4b6a212a30b4bc5
                        at libstd/panic.rs:361
                         - std::rt::lang_start_internal::h8b001b4244930d51
                        at libstd/rt.rs:58
  13:     0x55fa0ebaae71 - std::rt::lang_start::h1b1de624209f414a
                        at /checkout/src/libstd/rt.rs:74
  14:     0x55fa0ebaacbd - main
  15:     0x7f9946132f29 - __libc_start_main
  16:     0x55fa0eba9b79 - _start
  17:                0x0 - <unknown>'''
```

## Usage

```rust no_run
use human_panic::setup_panic;

fn main() {
   setup_panic!();

   println!("A normal log message");
   panic!("OMG EVERYTHING IS ON FIRE!!!")
}
```

It only displays a human-friendly panic message in release mode:

```sh
$ cargo run --release
```

## Installation

```sh
$ cargo add human-panic
```

## License

[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/human-panic.svg?style=flat-square
[2]: https://crates.io/crates/human-panic
[3]: https://img.shields.io/travis/rust-cli/human-panic.svg?style=flat-square
[4]: https://travis-ci.org/rust-cli/human-panic
[5]: https://img.shields.io/crates/d/human-panic.svg?style=flat-square
[6]: https://crates.io/crates/human-panic
[7]: https://docs.rs/human-panic/badge.svg
[8]: https://docs.rs/human-panic
