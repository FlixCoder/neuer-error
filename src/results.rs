//! Helpers on `Result` types for conversion and context addition.

use ::alloc::borrow::Cow;

use crate::{
	NeuErr,
	error::ProvideContext,
	features::{AnyDebugSendSync, ErrorSendSync},
};

/// Helper on our [`Result`](crate::Result)s for context addition and modification.
pub trait NeuErrResultExt<T, M>: Sized {
	/// Add human context to the error.
	#[track_caller]
	fn context<C>(self, context: C) -> Result<T, NeuErr<ProvideContext>>
	where
		C: Into<Cow<'static, str>>;

	/// Add human context to the error via a closure.
	#[track_caller]
	fn context_with<F, C>(self, context_fn: F) -> Result<T, NeuErr<ProvideContext>>
	where
		F: FnOnce() -> C,
		C: Into<Cow<'static, str>>;

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
	fn attach_with<F, C>(self, context_fn: F) -> Self
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static;

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
	fn attach_override_with<F, C>(self, context_fn: F) -> Self
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static;
}

impl<T, M> NeuErrResultExt<T, M> for Result<T, NeuErr<M>> {
	#[track_caller]
	fn context<C>(self, context: C) -> Result<T, NeuErr<ProvideContext>>
	where
		C: Into<Cow<'static, str>>,
	{
		// Cannot use `map_err` because closures cannot have `#[track_caller]` yet.
		match self {
			Ok(value) => Ok(value),
			Err(err) => Err(err.context(context)),
		}
	}

	#[track_caller]
	fn context_with<F, C>(self, context_fn: F) -> Result<T, NeuErr<ProvideContext>>
	where
		F: FnOnce() -> C,
		C: Into<Cow<'static, str>>,
	{
		// Cannot use `map_err` because closures cannot have `#[track_caller]` yet.
		match self {
			Ok(value) => Ok(value),
			Err(err) => Err(err.context(context_fn())),
		}
	}

	fn attach<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| err.attach(context))
	}

	fn attach_with<F, C>(self, context_fn: F) -> Self
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| err.attach(context_fn()))
	}

	fn attach_override<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| err.attach_override(context))
	}

	fn attach_override_with<F, C>(self, context_fn: F) -> Self
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| err.attach_override(context_fn()))
	}
}


/// Helper on `Result`s with external `Error`s for conversion to our `NeuErr`.
pub trait ConvertResult<T, E>: Sized {
	/// Add human context to the error.
	#[track_caller]
	fn context<C>(self, context: C) -> Result<T, NeuErr<ProvideContext>>
	where
		C: Into<Cow<'static, str>>;

	/// Add human context to the error via a closure.
	#[track_caller]
	fn context_with<F, C>(self, context_fn: F) -> Result<T, NeuErr<ProvideContext>>
	where
		F: FnOnce(&E) -> C,
		C: Into<Cow<'static, str>>;

	/// Add machine context to the error.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	fn attach<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static;

	/// Add machine context to the error via a closure.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	fn attach_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce(&E) -> C,
		C: AnyDebugSendSync + 'static;

	/// Set machine context in the error.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	fn attach_override<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static;

	/// Set machine context in the error via a closure.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	fn attach_override_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce(&E) -> C,
		C: AnyDebugSendSync + 'static;
}

impl<T, E> ConvertResult<T, E> for Result<T, E>
where
	E: ErrorSendSync + 'static,
{
	#[track_caller]
	fn context<C>(self, context: C) -> Result<T, NeuErr<ProvideContext>>
	where
		C: Into<Cow<'static, str>>,
	{
		// Cannot use `map_err` because closures cannot have `#[track_caller]` yet.
		match self {
			Ok(value) => Ok(value),
			Err(err) => Err(NeuErr::from_source(err).context(context)),
		}
	}

	#[track_caller]
	fn context_with<F, C>(self, context_fn: F) -> Result<T, NeuErr<ProvideContext>>
	where
		F: FnOnce(&E) -> C,
		C: Into<Cow<'static, str>>,
	{
		// Cannot use `map_err` because closures cannot have `#[track_caller]` yet.
		match self {
			Ok(value) => Ok(value),
			Err(err) => {
				let context = context_fn(&err);
				Err(NeuErr::from_source(err).context(context))
			}
		}
	}

	fn attach<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| NeuErr::from_source(err).attach(context))
	}

	fn attach_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce(&E) -> C,
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| {
			let attach = context_fn(&err);
			NeuErr::from_source(err).attach(attach)
		})
	}

	fn attach_override<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| NeuErr::from_source(err).attach_override(context))
	}

	fn attach_override_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce(&E) -> C,
		C: AnyDebugSendSync + 'static,
	{
		self.map_err(|err| {
			let attach = context_fn(&err);
			NeuErr::from_source(err).attach_override(attach)
		})
	}
}


/// Helper on `Option`s for conversion to our `Result`s.
pub trait ConvertOption<T>: Sized {
	/// Convert `None` to an error and add human context to the error.
	#[track_caller]
	fn context<C>(self, context: C) -> Result<T, NeuErr<ProvideContext>>
	where
		C: Into<Cow<'static, str>>;

	/// Convert `None` to an error and add human context to the error via a closure.
	#[track_caller]
	fn context_with<F, C>(self, context_fn: F) -> Result<T, NeuErr<ProvideContext>>
	where
		F: FnOnce() -> C,
		C: Into<Cow<'static, str>>;

	/// Convert `None` to an error and add machine context to the error.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	fn attach<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static;

	/// Convert `None` to an error and add machine context to the error via a closure.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	fn attach_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static;

	/// Convert `None` to an error and set machine context in the error.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	fn attach_override<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static;

	/// Convert `None` to an error and set machine context in the error via a closure.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	fn attach_override_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static;
}

impl<T> ConvertOption<T> for Option<T> {
	#[track_caller]
	fn context<C>(self, context: C) -> Result<T, NeuErr<ProvideContext>>
	where
		C: Into<Cow<'static, str>>,
	{
		// Cannot use `ok_or_else` because closures cannot have `#[track_caller]` yet.
		match self {
			Some(value) => Ok(value),
			None => Err(NeuErr::new(context)),
		}
	}

	#[track_caller]
	fn context_with<F, C>(self, context_fn: F) -> Result<T, NeuErr<ProvideContext>>
	where
		F: FnOnce() -> C,
		C: Into<Cow<'static, str>>,
	{
		// Cannot use `ok_or_else` because closures cannot have `#[track_caller]` yet.
		match self {
			Some(value) => Ok(value),
			None => {
				let context = context_fn();
				Err(NeuErr::new(context))
			}
		}
	}

	fn attach<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.ok_or_else(|| NeuErr::default().attach(context))
	}

	fn attach_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		self.ok_or_else(|| {
			let attach = context_fn();
			NeuErr::default().attach(attach)
		})
	}

	fn attach_override<C>(self, context: C) -> Result<T, NeuErr>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.ok_or_else(|| NeuErr::default().attach_override(context))
	}

	fn attach_override_with<F, C>(self, context_fn: F) -> Result<T, NeuErr>
	where
		F: FnOnce() -> C,
		C: AnyDebugSendSync + 'static,
	{
		self.ok_or_else(|| {
			let attach = context_fn();
			NeuErr::default().attach_override(attach)
		})
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
