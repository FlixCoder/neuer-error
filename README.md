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

## Development

## Minimum supported Rust version (MSRV)

Currently, I am always using the latest Rust version and do not put in any effort to keep the MSRV. Please open an issue in case you need a different policy, I might consider changing the policy.

## License

Licensed under the MIT license. All contributors must agree to publish under this license.
