//! Library for errors in both libraries and applications with:
//! - Information for machines to use for error handling.
//! - Contextual information for humans to receive helpful error messages.
//! - Comfortable, low-boilerplate API, that encourages (or enforces) additional context for errors
//!   along the way.

mod error;
mod macros;
mod results;

pub use self::{
	error::{ErrorStatus, HeapError},
	results::{ConvertResult, ResultExt},
};

/// Result type alias using the crate's `HeapError` type.
///
/// It often makes sense to make your own `Error` and `Result` aliases for your specific error
/// kind.
pub type Result<T, K = ()> = ::core::result::Result<T, HeapError<K>>;

#[cfg(test)]
mod tests;
