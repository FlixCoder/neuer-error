//! Typical error handling for libraries.
#![allow(
	dead_code,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	reason = "Example"
)]

use ::contextual_errors::Result;

/// Library creators need to think about what information would be interesting for humans and what
/// machine information is necessary to handle errors programmatically.
mod library {
	use ::contextual_errors::{CtxError, Result, traits::*};

	/// Kinds of errors that are interesting to match on for library users.
	/// If it is only interesting to humans, it can be iin the context instead.
	#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
	#[non_exhaustive]
	pub enum ErrorKind {
		NotFound,
		InvalidInput,
	}

	/// Should the error be retried?
	#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
	pub enum Retryable {
		No,
		Yes,
	}

	fn do_something_internal() -> Result<()> {
		Err(CtxError::new("Error occurred internally")
			.attach(ErrorKind::InvalidInput)
			.attach(Retryable::No))
	}

	pub fn do_something() -> Result<()> {
		do_something_internal().context("Operation failed")
	}
}

fn main() -> Result<()> {
	library::do_something()?;
	Ok(())
}
