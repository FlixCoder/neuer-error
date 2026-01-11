//! The error that can be whatever you want (it is Mr. Neuer). In every case (hopefully). NO AI
//! SLOP!
//!
//! An error handling library designed to be:
//!
//! - Useful in both libraries and applications, containing human and machine information.
//! - Ergonomic, low-boilerplate and comfortable, while still adhering best-practices and providing
//!   all necessary infos.
//! - Flexible in interfacing with other error handling libraries.
//!
//! ## Features/Highlights
//!
//! - Most importantly: error messages, that are helpful for debugging. By default it uses source
//!   locations instead of backtraces, which is often easier to follow, more efficient and works
//!   without debug info.
//! - Discoverable, typed context getters without generic soup, type conversions and conflicts.
//! - Works with std and no-std, but requires a global allocator.
//! - Compatible with non-Send/Sync environments, but also with Send/Sync environments ([per feature
//!   flag](#feature-flags)).
//! - Out of the box source error chaining.
//!
//! ## Why a new (German: neuer) error library?
//!
//! Long story, you can [view it here](https://github.com/FlixCoder/neuer-error/blob/main/why-another-lib.md).
//!
//! TLDR: I wasn't satisfied with my previous approach and existing libraries I know. And I was
//! inspired by a blog post to experiment myself with error handling design.
//!
//! ## Usage
//!
//! The best way to see how to use it for your use-case is to check out the [examples](https://github.com/FlixCoder/neuer-error/tree/main/examples).
//! Nevertheless, here is a quick demo:
//!
//! ```rust
//! # use neuer_error::{traits::*, CtxError, Result, provided_attachments};
//! // In library/module:
//! #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
//! pub enum Retryable { No, Yes }
//!
//! // Provide discoverable, typed information for library users.
//! provided_attachments!(
//!   retryable(single: Retryable) -> bool {
//!     |retryable| matches!(retryable, Some(Retryable::Yes))
//!   };
//! );
//!
//! fn do_something_internal() -> Result<()> {
//!   Err(CtxError::new("Error occurred internally")
//!     .attach(Retryable::No))
//! }
//!
//! pub fn do_something() -> Result<()> {
//!   do_something_internal().context("Operation failed")
//! }
//!
//! // In consumer/application:
//! fn main() {
//!   match do_something() {
//!     Ok(()) => {}
//!     Err(err) if err.retryable() => {
//!       eprintln!("Retryable error");
//!     }
//!     Err(_) => {
//!       eprintln!("Non-retryable error");
//!     }
//!   }
//! }
//! ```
//!
//! Run `cargo add neuer-error` to add the library to your project.
//!
//! ## Feature Flags
//!
//! **default** -> std, send, sync: Default selected features. Deactivate with
//! `default-features=false`.
//!
//! **std** (default): Enables use of `std`. Provides interaction with `ExitCode` termination.
//!
//! **send** (default): Requires all contained types to be `Send`, so that [`CtxError`] is also
//! `Send`.
//!
//! **sync** (default) -> send: Requires all contained types to be `Sync`, so that [`CtxError`] is
//! also `Sync`.
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc, clippy::alloc_instead_of_core)]

extern crate alloc;

mod error;
mod features;
mod macros;
mod results;

pub use self::{
	error::{CtxError, CtxErrorImpl},
	results::{ConvertOption, ConvertResult, CtxResultExt, ResultExt},
};

pub mod traits {
	//! All traits that need to be in scope for	comfortable usage.
	pub use crate::{ConvertOption as _, ConvertResult as _, CtxResultExt as _, ResultExt as _};
}

/// `Result` type alias using the crate's [`CtxError`] type.
pub type Result<T, E = CtxError> = ::core::result::Result<T, E>;

/// Create a `Result::Ok` value with [`CtxError`] as given error type.
#[inline(always)]
#[expect(non_snake_case, reason = "Mimics Result::Ok")]
pub const fn Ok<T>(value: T) -> Result<T> {
	Result::Ok(value)
}

#[cfg(test)]
mod tests;
