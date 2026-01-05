//! Helpers on `Result` types for conversion and context addition.

use crate::HeapError;

/// Helper on our `Result`s for context addition and modification.
pub trait ResultExt: Sized {
	/// Add context to the error directly.
	#[track_caller]
	#[must_use]
	fn context<C>(self, context: C) -> Self
	where
		C: ToString;

	/// Add context to the error via a closure.
	#[track_caller]
	#[must_use]
	#[inline]
	fn with_context<F, C>(self, context: F) -> Self
	where
		F: FnOnce() -> C,
		C: ToString,
	{
		ResultExt::context(self, context())
	}
}

impl<T, K> ResultExt for Result<T, HeapError<K>> {
	#[track_caller]
	fn context<C>(self, context: C) -> Self
	where
		C: ToString,
	{
		self.map_err(|err| err.context(context))
	}
}


/// Helper on external `Result`s for conversion to our `Result` type.
pub trait ConvertResult<T> {
	/// Convert the `Result` to one with the specified error kind.
	fn error_kind<K>(self, kind: K) -> Result<T, HeapError<K>>;
}

impl<T, E> ConvertResult<T> for Result<T, E>
where
	E: ::core::error::Error + 'static,
{
	fn error_kind<K>(self, kind: K) -> Result<T, HeapError<K>> {
		self.map_err(|err| HeapError::new_with_source(kind, Box::new(err)))
	}
}
