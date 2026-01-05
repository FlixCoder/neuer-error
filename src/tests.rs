//! Tests for the error types.

use ::std::fmt::Display;

use crate::{ConvertResult, ResultExt};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum ErrorKind {
	NotFound,
}

impl Display for ErrorKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::NotFound => "not found",
		};
		f.write_str(s)
	}
}

fn failing_io() -> Result<(), std::io::Error> {
	Err(std::io::Error::new(std::io::ErrorKind::NotFound, "IO"))
}

fn failing() -> crate::Result<(), ErrorKind> {
	failing_io().error_kind(ErrorKind::NotFound).context("test")
}

#[test]
fn error_context() {
	let err = failing().unwrap_err();
}
