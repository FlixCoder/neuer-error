//! Helpers on `Result` types for conversion and context addition.

use crate::{
	CtxError,
	features::{AnyDebugSendSync, ErrorSendSync},
};

/// Helper on our [`Result`](crate::Result)s for context addition and modification.
pub trait CtxResultExt: Sized {
	/// Add human context to the error.
	#[track_caller]
	#[must_use]
	fn context<C>(self, context: C) -> Self
	where
		C: ToString;

	/// Add human context to the error via a closure.
	#[track_caller]
	#[must_use]
	#[inline]
	fn context_with<F, C>(self, context_fn: F) -> Self
	where
		F: FnOnce() -> C,
		C: ToString,
	{
		CtxResultExt::context(self, context_fn())
	}

	/// Add machine context to the error.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	#[must_use]
	fn attach<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static;

	/// Add machine context to the error via a closure.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	#[must_use]
	#[inline]
	fn attach_with<F, C>(self, context_fn: F) -> Self
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		CtxResultExt::attach(self, context_fn())
	}

	/// Set machine context in the error.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	#[must_use]
	fn attach_override<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static;

	/// Set machine context in the error via a closure.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	#[must_use]
	#[inline]
	fn attach_override_with<F, C>(self, context_fn: F) -> Self
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		CtxResultExt::attach_override(self, context_fn())
	}
}

impl<T> CtxResultExt for Result<T, CtxError> {
	#[track_caller]
	#[inline]
	fn context<C>(self, context: C) -> Self
	where
		C: ToString,
	{
		self.map_err(|err| err.context(context))
	}

	#[inline]
	fn attach<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| err.attach(context))
	}

	#[inline]
	fn attach_override<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| err.attach_override(context))
	}
}


/// Helper on `Result`s with external `Error`s for conversion to our `CtxError`.
pub trait ConvertResult<T>: Sized {
	/// Add human context to the error.
	#[track_caller]
	fn context<C>(self, context: C) -> Result<T, CtxError>
	where
		C: ToString;

	/// Add human context to the error via a closure.
	#[track_caller]
	#[inline]
	fn context_with<F, C>(self, context_fn: F) -> Result<T, CtxError>
	where
		F: FnOnce() -> C,
		C: ToString,
	{
		ConvertResult::context(self, context_fn())
	}

	/// Add machine context to the error.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	fn attach<C>(self, context: C) -> Result<T, CtxError>
	where
		C: AnyDebugSendSync + 'static;

	/// Add machine context to the error via a closure.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	#[inline]
	fn attach_with<F, C>(self, context_fn: F) -> Result<T, CtxError>
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		ConvertResult::attach(self, context_fn())
	}

	/// Set machine context in the error.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	fn attach_override<C>(self, context: C) -> Result<T, CtxError>
	where
		C: AnyDebugSendSync + 'static;

	/// Set machine context in the error via a closure.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	#[inline]
	fn attach_override_with<F, C>(self, context_fn: F) -> Result<T, CtxError>
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		ConvertResult::attach_override(self, context_fn())
	}
}

impl<T, E> ConvertResult<T> for Result<T, E>
where
	E: ErrorSendSync + 'static,
{
	#[track_caller]
	#[inline]
	fn context<C>(self, context: C) -> Result<T, CtxError>
	where
		C: ToString,
	{
		self.map_err(|err| CtxError::new_with_source(context, err))
	}

	#[inline]
	fn attach<C>(self, context: C) -> Result<T, CtxError>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| CtxError::from_source(err).attach(context))
	}

	#[inline]
	fn attach_override<C>(self, context: C) -> Result<T, CtxError>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| CtxError::from_source(err).attach_override(context))
	}
}


/// Helpers on `Result`s.
pub trait ResultExt<T, E> {
	/// Consumes the error from the `Result` and pushes it into the provided collection.
	fn or_collect<C>(self, collection: &mut C) -> Option<T>
	where
		C: Extend<E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
	#[inline]
	fn or_collect<C>(self, collection: &mut C) -> Option<T>
	where
		C: Extend<E>,
	{
		match self {
			Ok(value) => Some(value),
			Err(err) => {
				collection.extend(core::iter::once(err));
				None
			}
		}
	}
}
