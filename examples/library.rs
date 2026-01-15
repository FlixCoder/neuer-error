//! Typical error handling for libraries.
#![allow(
	dead_code,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	reason = "Example"
)]

use ::neuer_error::Result;

use self::library::NeuErrAttachments;

/// Library creators need to think about what information would be interesting for humans and what
/// machine information is necessary to handle errors programmatically.
///
/// When providing attachments, library authors should make use of the `provided_attachments!`
/// macro!
mod library {
	use ::neuer_error::{NeuErr, Result, provided_attachments, traits::*};

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

	// Provide discoverable, typed information for library users.
	provided_attachments!(
		kind(single: ErrorKind) -> Option<ErrorKind> { |kind| kind.copied() };
		retryable(single: Retryable) -> bool {
			|retryable| matches!(retryable, Some(Retryable::Yes))
		};
	);

	/// Implement your errors while attaching machine-targeted information.
	fn do_something_internal() -> Result<()> {
		Err(NeuErr::new("Error occurred internally")
			.attach(ErrorKind::InvalidInput)
			.attach(Retryable::No))
	}

	/// Alose provide human-targeted context when propagating errors.
	pub fn do_something() -> Result<()> {
		do_something_internal().context("Operation failed")
	}
}

fn main() -> Result<()> {
	// Library users can use the machine context.
	match library::do_something() {
		Ok(()) => {}
		Err(err) if err.retryable() => {
			eprintln!("Retryable error");
		}
		Err(_) => {
			eprintln!("Non-retryable error");
		}
	}

	// Or just pass on the error.
	library::do_something()?;
	Ok(())
}
