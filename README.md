# Neuer Error

[![crates.io page](https://img.shields.io/crates/v/neuer-error.svg)](https://crates.io/crates/neuer-error)
[![docs.rs page](https://docs.rs/neuer-error/badge.svg)](https://docs.rs/neuer-error/)
![license: MIT](https://img.shields.io/crates/l/neuer-error.svg)

The error that can be whatever you want (it is Mr. Neuer). In every case (hopefully). NO AI SLOP!

An error handling library designed to be:

- Useful in both [libraries](examples/library.rs) and [applications](examples), containing human and machine information.
- Ergonomic, low-boilerplate and comfortable, while still adhering best-practices and providing all necessary infos.
- Flexible in interfacing with other error handling libraries.

## Features

- Most importantly: error messages, that are helpful for debugging. By default it uses source locations instead of backtraces, which is often easier to follow, more efficient and works without debug info.
- Discoverable, typed context getters without generic soup, type conversions and conflicts.
- Works with std and no-std, but requires a global allocator. [See example](examples/embedded-no-std.rs).
- Compatible with non-Send/Sync environments, but also with Send/Sync environments (per feature flag).
- Out of the box source error chaining.

## Why a new (German: neuer) error library?

Long story, you can [view it here](why-another-lib.md).
TLDR: I wasn't satisfied with my previous approach and existing libraries I know. And I was inspired by a blog post to experiment myself with error handling design.

## Usage

The best way to see how to use it for your use-case is to check out the [examples](examples).
Nevertheless, here is a quick demo:

```rust
// In library/module:
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Retryable { No, Yes }

// Provide discoverable, typed information for library users.
provided_attachments!(
  retryable(single: Retryable) -> bool {
    |retryable| matches!(retryable, Some(Retryable::Yes))
  };
);

fn do_something_internal() -> Result<()> {
  Err(CtxError::new("Error occurred internally")
    .attach(Retryable::No))
}

pub fn do_something() -> Result<()> {
  do_something_internal().context("Operation failed")
}

// In consumer/application:
fn main() {
  match do_something() {
    Ok(()) => {}
    Err(err) if err.retryable() => {
      eprintln!("Retryable error");
    }
    Err(_) => {
      eprintln!("Non-retryable error");
    }
  }
}
```

Run `cargo add neuer-error` to add the library to your project.

## Development

If you want to contribute or have questions, feel free to open issues :)
Always better to ask before investing too much effort into PRs that I won't accept.

Running all the checks is quite simple:

1. Install [cargo-make](https://github.com/sagiegurari/cargo-make): `cargo install cargo-make`.
2. Optional, but recommended: Put `search_project_root = true` into cargo-make's user configuration, so that `cargo make` can be run from sub-folders.
3. From the project directory, you can run the following tasks:
   - **Run all checks that are done in CI**: `cargo make ci` or just `cargo make`.
   - **Format code**: `cargo make format`.
   - **Check formatting**: `cargo make formatting`.
   - **Run all tests via cargo test**: `cargo make test`.
   - **Run clippy for all feature sets, failing on any warnings**: `cargo make clippy`.

## Minimum supported Rust version (MSRV)

Currently, I am always using the latest Rust version and do not put in any effort to keep the MSRV. Please open an issue in case you need a different policy, I might consider changing the policy.

## License

Licensed under the MIT license. All contributors must agree to publish under this license.
