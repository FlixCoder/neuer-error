//! Error type implementation.

use ::alloc::{string::ToString, vec};
use ::core::{
	any::Any,
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	panic::Location,
};

use crate::features::{AnyDebugSendSync, Box, ErrorSendSync, String, Vec};

/// Error information for humans.
/// Error message with location information.
#[derive(Debug)]
pub(crate) struct HumanInfo {
	/// Message text.
	pub(crate) message: String,
	/// Location of occurrence.
	pub(crate) location: &'static Location<'static>,
}

/// Error information for machines.
/// Arbitrary, project specific types of information.
#[derive(Debug)]
pub(crate) struct MachineInfo {
	/// Attachment.
	pub(crate) attachment: Box<dyn AnyDebugSendSync>,
}

/// Context information, either machine or human.
/// Joined in a union type to save the space of another `Vec` in the error type.
#[derive(Debug)]
pub(crate) enum Info {
	/// Contextual information for humans.
	Human(HumanInfo),
	/// Contextual information for machines.
	Machine(MachineInfo),
}
// Ensure size of Context is as expected. Can be adjusted though.
const _: () = {
	assert!(size_of::<Info>() == 32);
};

/// Generic rich error type for use within `Result`s, for libraries and applications.
///
/// Add human context information, including code locations, via the `context` method.
/// Attach machine context information via the `attach` and `attach_override` methods.
///
/// ## Error Formatting
///
/// The normal `Debug` implementation (`"{err:?}"`) will print the error with multi-line formatting,
/// exactly how `Display` is doing it. The alternate `Debug` implementation (`"{err:#?}"`) will show
/// the pretty-printed usual debug representation of the internal types.
///
/// When using the `Display` implementation, the normal implementation (`"{err}"`) will use
/// multi-line formatting. You can use the alternate format (`{err:#}`) to get a compact single-line
/// version. instead of multi-line formatted.
#[derive(Default)]
pub struct CtxError(CtxErrorImpl);

/// Inner implementation of [`CtxError`] that implements [`Error`].
#[derive(Default)]
pub struct CtxErrorImpl {
	/// Contextual error information.
	infos: Vec<Info>,
	/// Source error.
	source: Option<Box<dyn ErrorSendSync>>,
}

impl Debug for CtxError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Debug::fmt(&self.0, f)
	}
}

impl Display for CtxError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self.0, f)
	}
}

impl Debug for CtxErrorImpl {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if f.alternate() {
			f.debug_struct("CtxError")
				.field("infos", &self.infos)
				.field("source", &self.source)
				.finish()
		} else {
			Display::fmt(self, f)
		}
	}
}

impl Display for CtxErrorImpl {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let mut human = self.contexts().peekable();
		if human.peek().is_none() {
			write!(f, "Unknown error")?;
		}
		while let Some(context) = human.next() {
			if f.alternate() {
				write!(f, "{} (at {})", context.message, context.location)?;
				if human.peek().is_some() {
					write!(f, "; ")?;
				}
			} else {
				writeln!(f, "{}", context.message)?;
				write!(f, "|- at {}", context.location)?;
				if human.peek().is_some() {
					writeln!(f)?;
					writeln!(f, "|")?;
				}
			}
		}

		#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
		let mut source = self.source.as_deref().map(|e| e as &(dyn Error + 'static));
		while let Some(err) = source {
			if f.alternate() {
				write!(f, "; caused by: {err}")?;
			} else {
				writeln!(f)?;
				writeln!(f, "|")?;
				write!(f, "|- caused by: {err}")?;
			}

			source = err.source();
		}

		Ok(())
	}
}

impl CtxError {
	/// Create new error.
	#[track_caller]
	#[must_use]
	#[inline]
	pub fn new<T: ToString>(message: T) -> Self {
		let infos = vec![Info::Human(HumanInfo {
			message: message.to_string(),
			location: Location::caller(),
		})];
		Self(CtxErrorImpl { infos, ..Default::default() })
	}

	/// Create new error from source error.
	#[track_caller]
	#[must_use]
	#[inline]
	pub fn new_with_source<T, E>(message: T, source: E) -> Self
	where
		T: ToString,
		E: ErrorSendSync + 'static,
	{
		let infos = vec![Info::Human(HumanInfo {
			message: message.to_string(),
			location: Location::caller(),
		})];
		Self(CtxErrorImpl { infos, source: Some(Box::new(source)) })
	}

	/// Convert source error.
	#[must_use]
	#[inline]
	pub fn from_source<E>(source: E) -> Self
	where
		E: ErrorSendSync + 'static,
	{
		Self(CtxErrorImpl { source: Some(Box::new(source)), ..Default::default() })
	}

	/// Add human context to the error.
	#[track_caller]
	#[must_use]
	#[inline]
	pub fn context<C>(self, context: C) -> Self
	where
		C: ToString,
	{
		Self(self.0.context(context))
	}

	/// Add machine context to the error.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	#[must_use]
	#[inline]
	pub fn attach<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		Self(self.0.attach(context))
	}

	/// Set machine context in the error.
	///
	/// This will override existing attachments of the same type. If you want to add attachments of
	/// the same type, use `attach` instead.
	#[must_use]
	pub fn attach_override<C>(self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		Self(self.0.attach_override(context))
	}

	/// Get an iterator over the human context infos.
	#[inline]
	#[cfg_attr(not(test), expect(unused, reason = "For consistency"))]
	pub(crate) fn contexts(&self) -> impl Iterator<Item = &'_ HumanInfo> {
		self.0.contexts()
	}

	/// Get an iterator over the machine context attachments of the given type.
	#[inline]
	pub fn attachments<C>(&self) -> impl Iterator<Item = &'_ C>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.0.attachments()
	}

	/// Get the machine context attachment of the given type.
	#[must_use]
	#[inline]
	pub fn attachment<C>(&self) -> Option<&C>
	where
		C: AnyDebugSendSync + 'static,
	{
		self.0.attachment()
	}

	/// Get the source error.
	#[must_use]
	#[inline]
	pub fn source(&self) -> Option<&(dyn ErrorSendSync + 'static)> {
		self.0.source.as_deref()
	}

	/// Unwrap this error into a [`CtxErrorImpl`] that implements [`Error`]. Note however, that it
	/// does not offer all of the functionality and might be unwieldy for other general purposes
	/// than interfacing with other error types.
	#[must_use]
	#[inline]
	pub fn into_error(self) -> CtxErrorImpl {
		self.0
	}
}

impl CtxErrorImpl {
	/// Wrap this error back into a [`CtxError`] that offers all of the functionality.
	#[must_use]
	#[inline]
	pub const fn wrap(self) -> CtxError {
		CtxError(self)
	}

	/// Add human context to the error.
	#[track_caller]
	#[must_use]
	#[inline]
	pub fn context<C>(mut self, context: C) -> Self
	where
		C: ToString,
	{
		let context = HumanInfo { message: context.to_string(), location: Location::caller() };
		self.infos.push(Info::Human(context));
		self
	}

	/// Add machine context to the error.
	///
	/// This will not override existing attachments. If you want to replace and override any
	/// existing attachments of the same type, use `attach_override` instead.
	#[must_use]
	#[inline]
	pub fn attach<C>(mut self, context: C) -> Self
	where
		C: AnyDebugSendSync + 'static,
	{
		let context = MachineInfo { attachment: Box::new(context) };
		self.infos.push(Info::Machine(context));
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
		let context = MachineInfo { attachment: Box::new(context) };
		self.infos.retain(|info| match info {
			Info::Human(_) => true,
			Info::Machine(ctx) => {
				#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
				let any = (ctx.attachment.as_ref()) as &(dyn Any + 'static);
				!any.is::<C>()
			}
		});
		self.infos.push(Info::Machine(context));
		self
	}

	/// Get an iterator over all context infos.
	#[inline]
	pub(crate) fn infos(&self) -> impl Iterator<Item = &'_ Info> {
		self.infos.iter().rev()
	}

	/// Get an iterator over the human context infos.
	#[inline]
	pub(crate) fn contexts(&self) -> impl Iterator<Item = &'_ HumanInfo> {
		self.infos().filter_map(|info| match info {
			Info::Human(info) => Some(info),
			_ => None,
		})
	}

	/// Get an iterator over the machine context attachments of the given type.
	#[inline]
	pub fn attachments<C>(&self) -> impl Iterator<Item = &'_ C>
	where
		C: AnyDebugSendSync + 'static,
	{
		#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
		self.infos()
			.filter_map(|info| match info {
				Info::Machine(info) => Some(info),
				_ => None,
			}) // Catch the newest attachment first.
			.map(|ctx| ctx.attachment.as_ref() as &(dyn Any + 'static))
			.filter_map(|ctx| ctx.downcast_ref())
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
}

impl From<CtxError> for CtxErrorImpl {
	fn from(err: CtxError) -> Self {
		err.0
	}
}

#[diagnostic::do_not_recommend]
impl<E> From<E> for CtxError
where
	E: ErrorSendSync + 'static,
{
	#[inline]
	fn from(err: E) -> Self {
		Self::from_source(err)
	}
}

impl Error for CtxErrorImpl {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		#[expect(trivial_casts, reason = "Not that trivial as it seems? False positive")]
		self.source.as_deref().map(|e| e as &(dyn Error + 'static))
	}
}

impl AsRef<dyn Error> for CtxError {
	fn as_ref(&self) -> &(dyn Error + 'static) {
		&self.0
	}
}

#[cfg(feature = "send")]
impl AsRef<dyn Error + Send> for CtxError {
	fn as_ref(&self) -> &(dyn Error + Send + 'static) {
		&self.0
	}
}

#[cfg(all(feature = "send", feature = "sync"))]
impl AsRef<dyn Error + Send + Sync> for CtxError {
	fn as_ref(&self) -> &(dyn Error + Send + Sync + 'static) {
		&self.0
	}
}

impl From<CtxError> for Box<dyn Error> {
	fn from(this: CtxError) -> Self {
		Box::new(this.into_error())
	}
}

#[cfg(feature = "send")]
impl From<CtxError> for Box<dyn Error + Send> {
	fn from(this: CtxError) -> Self {
		Box::new(this.into_error())
	}
}

#[cfg(all(feature = "send", feature = "sync"))]
impl From<CtxError> for Box<dyn Error + Send + Sync> {
	fn from(this: CtxError) -> Self {
		Box::new(this.into_error())
	}
}


#[cfg(feature = "std")]
impl std::process::Termination for CtxError {
	fn report(self) -> std::process::ExitCode {
		std::process::Termination::report(self.0)
	}
}

#[cfg(feature = "std")]
impl std::process::Termination for CtxErrorImpl {
	fn report(self) -> std::process::ExitCode {
		self.attachment::<std::process::ExitCode>()
			.copied()
			.unwrap_or(std::process::ExitCode::FAILURE)
	}
}
