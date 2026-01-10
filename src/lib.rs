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
	results::{ConvertResult, CtxResultExt, ResultExt},
};

/// `Result` type alias using the crate's [`CtxError`] type.
pub type Result<T, E = CtxError> = ::core::result::Result<T, E>;

pub mod traits {
	//! All traits that need to be in scope for	comfortable usage.
	pub use crate::{ConvertResult as _, CtxResultExt as _, ResultExt as _};
}

#[cfg(test)]
mod tests;
