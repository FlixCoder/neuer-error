//! Tests for the error types.

use ::core::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::{ConvertResult, CtxResultExt, Result};

#[allow(dead_code, reason = "Example")]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
enum ErrorStatus {
	/// Not retryable.
	#[default]
	Permanent,
	/// Retryable.
	Temporary,
	/// Was already retried, but still failed again.
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

#[derive(Debug)]
struct ComplexInfo {
	_weird_stuff: Box<dyn Debug + Send + Sync>,
}

fn failing_io() -> Result<(), std::io::Error> {
	Err(std::io::Error::new(std::io::ErrorKind::NotFound, "IO"))
}

fn failing() -> Result<()> {
	failing_io()
		.context("test")
		.attach_override(ErrorStatus::Temporary)
		.attach(ComplexInfo { _weird_stuff: Box::new("") })
}

#[test]
fn error_context() {
	_ = failing().unwrap_err();
}
