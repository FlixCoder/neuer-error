//! Error type implementation.

use ::core::{
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use ::std::{
	backtrace::{Backtrace, BacktraceStatus},
	panic::Location,
};

/// Meaning of the error in terms of error handling.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum ErrorStatus {
	/// Error is permanent and must not be retried.
	#[default]
	Permanent,
	/// Error is temporary and may be retried.
	Temporary,
	/// Error is temporary, but was already retried without success.
	Persistent,
}

impl Display for ErrorStatus {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let s = match self {
			Self::Permanent => "permanent",
			Self::Temporary => "temporary",
			Self::Persistent => "persistent",
		};
		f.write_str(s)
	}
}

/// Error message with location information.
#[derive(Debug)]
struct ErrorMessage {
	/// Message text.
	message: String,
	/// Location of occurrence.
	location: &'static Location<'static>,
}

/// Generic rich error type for use within `Result`s, for libraries and applications.
///
/// When using the `Display` implementation, you can use `{:#}` to get a compact single-line version
/// instead of multi-line formatted.
///
/// It often makes sense to make your own `Error` and `Result` aliases for your specific error kind.
#[derive(Debug)]
pub struct HeapError<K> {
	/// Kind of error, if given.
	kind: Option<K>,
	/// Error status.
	status: ErrorStatus,
	/// Contextual information for humans.
	context: Vec<ErrorMessage>,
	/// Source error this was constructed from.
	source: Option<Box<dyn Error>>,
	/// Optional backtrace at main creation, only captured without error kind.
	backtrace: Option<Box<Backtrace>>,
}

impl<K: Display> Display for HeapError<K> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "[{}]: ", self.status)?;
		if let Some(kind) = self.kind.as_ref() {
			kind.fmt(f)?;
		} else {
			f.write_str("Unknown error")?;
		}

		for context in self.context.iter().rev() {
			if f.alternate() {
				write!(f, "; {} at {}", context.message, context.location)?;
			} else {
				writeln!(f)?;
				writeln!(f)?;
				writeln!(f, "|- {}", context.message)?;
				write!(f, "|- at {}", context.location)?;
			}
		}

		let mut source = self.source.as_deref();
		while let Some(err) = source {
			if f.alternate() {
				write!(f, "; caused by {err}")?;
			} else {
				writeln!(f)?;
				writeln!(f)?;
				writeln!(f, "|- caused by {err}")?;
			}

			source = err.source();
		}

		if !f.alternate()
			&& let Some(backtrace) = self.backtrace.as_deref()
		{
			writeln!(f)?;
			writeln!(f)?;
			writeln!(f, "|- Backtrace:")?;
			write!(f, "{backtrace}")?;
		}

		Ok(())
	}
}

impl<K: Debug + Display> Error for HeapError<K> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.source.as_deref()
	}
}

impl<K> Default for HeapError<K> {
	fn default() -> Self {
		let backtrace = Backtrace::capture();
		Self {
			kind: None,
			status: ErrorStatus::default(),
			context: vec![],
			source: None,
			backtrace: (backtrace.status() == BacktraceStatus::Captured)
				.then(move || Box::new(backtrace)),
		}
	}
}

impl<K> HeapError<K> {
	/// Create new error.
	#[must_use]
	pub fn new(kind: K) -> Self {
		Self {
			kind: Some(kind),
			status: ErrorStatus::default(),
			context: vec![],
			source: None,
			backtrace: None,
		}
	}

	/// Create new error from source error.
	#[must_use]
	pub fn new_with_source(kind: K, source: Box<dyn Error>) -> Self {
		Self {
			kind: Some(kind),
			status: ErrorStatus::default(),
			context: vec![],
			source: Some(source),
			backtrace: None,
		}
	}

	/// Convert source error.
	#[must_use]
	pub fn from_source(source: Box<dyn Error>) -> Self {
		let backtrace = Backtrace::capture();
		Self {
			kind: None,
			status: ErrorStatus::default(),
			context: vec![],
			source: Some(source),
			backtrace: (backtrace.status() == BacktraceStatus::Captured)
				.then(move || Box::new(backtrace)),
		}
	}

	/// Get the error kind.
	pub const fn kind(&self) -> Option<&K> {
		self.kind.as_ref()
	}

	/// Get the error status.
	pub const fn status(&self) -> ErrorStatus {
		self.status
	}

	/// Get the backtrace.
	pub fn backtrace(&self) -> Option<&Backtrace> {
		self.backtrace.as_deref()
	}

	/// Set the error status.
	pub const fn set_status(&mut self, status: ErrorStatus) {
		self.status = status;
	}

	/// Add context to the error.
	#[track_caller]
	#[must_use]
	pub fn context<C>(mut self, context: C) -> Self
	where
		C: ToString,
	{
		let context = ErrorMessage { message: context.to_string(), location: Location::caller() };
		self.context.push(context);
		self
	}
}
