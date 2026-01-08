//! Error type implementation.

use ::core::{
	any::Any,
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	ops::{Deref, DerefMut},
	panic::Location,
};

use crate::features::{AnyDebugSendSync, Container, ErrorSendSync, Stack};

/// Error information for humans.
/// Error message with location information.
#[derive(Debug)]
struct HumanInfo {
	/// Message text.
	message: String,
	/// Location of occurrence.
	location: &'static Location<'static>,
}

/// Error information for machines.
/// Arbitrary, project specific types of information.
#[derive(Debug)]
struct MachineInfo {
	/// Attachment.
	attachment: Container<dyn AnyDebugSendSync>,
}

/// Generic rich error type for use within `Result`s, for libraries and applications.
///
/// When using the `Display` implementation, you can use `{:#}` to get a compact single-line version
/// instead of multi-line formatted.
#[derive(Debug, Default)]
pub struct CtxError {
	/// Contextual information for humans.
	human: Stack<HumanInfo>,
	/// Contextual information for machines.
	machine: Stack<MachineInfo>,
	/// Source error.
	source: Option<Container<dyn ErrorSendSync>>,
}

impl Display for CtxError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if self.human.is_empty() {
			write!(f, "Unknown error")?;
		}

		let mut context_iter = self.human.iter().rev().peekable();
		while let Some(context) = context_iter.next() {
			if f.alternate() {
				write!(f, "{} (at {})", context.message, context.location)?;
				if context_iter.peek().is_some() {
					write!(f, "; ")?;
				}
			} else {
				writeln!(f, "{}", context.message)?;
				write!(f, "|- at {}", context.location)?;
				if context_iter.peek().is_some() {
					writeln!(f)?;
					writeln!(f, "|")?;
				}
			}
		}

		#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
		let mut source = self.source.as_deref().map(|e| e as &(dyn Error + 'static));
		while let Some(err) = source {
			if f.alternate() {
				write!(f, "; caused by {err}")?;
			} else {
				writeln!(f)?;
				writeln!(f, "|")?;
				writeln!(f, "|- caused by {err}")?;
			}

			source = err.source();
		}

		Ok(())
	}
}

#[expect(clippy::vec_init_then_push, reason = "Will be different type without heap allocation")]
impl CtxError {
	/// Create new error.
	#[track_caller]
	#[must_use]
	pub fn new<T: ToString>(message: T) -> Self {
		let mut human = Stack::new();
		human.push(HumanInfo { message: message.to_string(), location: Location::caller() });
		Self { human, ..Default::default() }
	}

	/// Create new error from source error.
	#[track_caller]
	#[must_use]
	pub fn new_with_source<T, E>(message: T, source: E) -> Self
	where
		T: ToString,
		E: ErrorSendSync + 'static,
	{
		let mut human = Stack::new();
		human.push(HumanInfo { message: message.to_string(), location: Location::caller() });
		Self { human, source: Some(Container::new(source)), ..Default::default() }
	}

	/// Convert source error.
	#[must_use]
	pub fn from_source<E>(source: E) -> Self
	where
		E: ErrorSendSync + 'static,
	{
		Self { source: Some(Container::new(source)), ..Default::default() }
	}

	/// Add human context to the error.
	#[track_caller]
	#[must_use]
	pub fn context<C>(mut self, context: C) -> Self
	where
		C: ToString,
	{
		let context = HumanInfo { message: context.to_string(), location: Location::caller() };
		self.human.push(context);
		self
	}

	/// Add machine context to the error.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	#[must_use]
	pub fn attach<C>(mut self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		let context = MachineInfo { attachment: Container::new(context) };
		self.machine.push(context);
		self
	}

	/// Set machine context in the error.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	#[must_use]
	pub fn attach_override<C>(mut self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		let context = MachineInfo { attachment: Container::new(context) };
		self.machine.retain(|ctx| {
			#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
			let any = &ctx.attachment as &(dyn Any + 'static);
			!any.is::<C>()
		});
		self.machine.push(context);
		self
	}

	/// Get the machine context attachment of the given type.
	#[must_use]
	#[inline]
	pub fn attachment<C>(&self) -> Option<&C>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.attachments().next()
	}

	/// Get all machine context attachments (iterator) of the given type.
	pub fn attachments<C>(&self) -> impl Iterator<Item = &'_ C>
	where
		C: AnyDebugSendSync + 'static,
	{
		#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
		self.machine
			.iter()
			.map(|ctx| ctx.attachment.as_ref() as &(dyn Any + 'static))
			.filter_map(|ctx| ctx.downcast_ref())
	}

	/// Wrap this error into a [`CtxErrorImpl`] that implements [`Error`].
	#[must_use]
	#[inline]
	pub const fn into_error(self) -> CtxErrorImpl {
		CtxErrorImpl(self)
	}
}

impl<E> From<E> for CtxError
where
	E: ErrorSendSync + 'static,
{
	#[inline]
	fn from(err: E) -> Self {
		CtxError::from_source(err)
	}
}

/// Wrapper for [`CtxError`] that implements [`Error`].
#[derive(Debug, Default)]
pub struct CtxErrorImpl(pub CtxError);

impl From<CtxError> for CtxErrorImpl {
	#[inline]
	fn from(err: CtxError) -> Self {
		Self(err)
	}
}

impl Deref for CtxErrorImpl {
	type Target = CtxError;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for CtxErrorImpl {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Display for CtxErrorImpl {
	#[inline]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self.0, f)
	}
}

impl Error for CtxErrorImpl {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
		self.0.source.as_deref().map(|e| e as &(dyn Error + 'static))
	}
}

impl CtxErrorImpl {
	/// Unwrap into the inner [`CtxError`].
	#[must_use]
	#[inline]
	pub fn into_inner(self) -> CtxError {
		self.0
	}
}
