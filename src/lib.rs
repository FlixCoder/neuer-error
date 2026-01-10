//! Library for errors in both libraries and applications with:
//! - Information for machines to use for error handling.
//! - Contextual information for humans to receive helpful error messages.
//! - Comfortable, low-boilerplate API, that encourages (or enforces) additional context for errors
//!   along the way.
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::std_instead_of_core, clippy::std_instead_of_alloc, clippy::alloc_instead_of_core)]

#[cfg(feature = "alloc")]
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
