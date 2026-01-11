//! Crate tests.

use ::alloc::{format, vec::Vec};
use ::core::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult},
	panic::Location,
};
use ::regex::Regex;

use crate::*;


#[test]
fn debug_impl() {
	let error = level2().unwrap_err().attach(0);
	let normal = format!("{error:?}");
	let alternate = format!("{error:#?}");

	let matcher = Regex::new(r"Level 2 error\n|- at src/tests\.rs:\d+:\d+\n|\nLevel 1 error\n|- at src/tests\.rs:\d+:\d+\n|\nLevel 0 error\n|- at src/tests\.rs:\d+:\d+\n|\n|- caused by: SourceError occurred\n|\n|- caused by: provided string was not `true` or `false`").expect("failed compiling regex");
	assert!(matcher.is_match(&normal), "Found: {normal}");

	let matcher = Regex::new(
		r#"
CtxError \{
    infos: \[
        Human\(
            HumanInfo \{
                message: "Level 0 error",
                location: Location \{
                    file: "src/tests\.rs",
                    line: \d+,
                    column: \d+,
                \},
            \},
        \),
        Human\(
            HumanInfo \{
                message: "Level 1 error",
                location: Location \{
                    file: "src/tests\.rs",
                    line: \d+,
                    column: \d+,
                \},
            \},
        \),
        Human\(
            HumanInfo \{
                message: "Level 2 error",
                location: Location \{
                    file: "src/tests\.rs",
                    line: \d+,
                    column: \d+,
                \},
            \},
        \),
        Machine\(
            MachineInfo \{
                attachment: 0,
            \},
        \),
    \],
    source: Some\(
        SourceError\(
            ParseBoolError,
        \),
    \),
\}
		"#
		.trim(),
	)
	.expect("failed compiling regex");
	assert!(matcher.is_match(&alternate), "Found: {alternate}");
}

#[test]
fn display_impl() {
	let error = level2().unwrap_err().attach(0);
	let normal = format!("{error}");
	let alternate = format!("{error:#}");

	let matcher = Regex::new(r"Level 2 error\n|- at src/tests\.rs:\d+:\d+\n|\nLevel 1 error\n|- at src/tests\.rs:\d+:\d+\n|\nLevel 0 error\n|- at src/tests\.rs:\d+:\d+\n|\n|- caused by: SourceError occurred\n|\n|- caused by: provided string was not `true` or `false`").expect("failed compiling regex");
	assert!(matcher.is_match(&normal), "Found: {normal}");

	let matcher = Regex::new(r"Level 2 error \(at src/tests\.rs:\d+:\d+\); Level 1 error \(at src/tests\.rs:\d+:\d+\); Level 0 error \(at src/tests\.rs:\d+:\d+\); caused by: SourceError occurred; caused by: provided string was not `true` or `false`").expect("failed compiling regex");
	assert!(matcher.is_match(&alternate), "Found: {alternate}");
}

#[test]
fn error_wrapper() {
	let error = level1().unwrap_err().into_error();
	assert!(Error::source(&error).is_some());

	let error = error.into_inner();
	assert!(error.source().is_some());
}

#[test]
fn context() {
	let error = CtxError::new("0").context("1").context("2");
	let mut numbers = error.contexts().map(|ctx| ctx.message.parse::<u8>().unwrap());
	assert_eq!(numbers.next(), Some(2));
	assert_eq!(numbers.next(), Some(1));
	assert_eq!(numbers.next(), Some(0));
	assert_eq!(numbers.next(), None);
}

#[test]
fn context_correct_locations() {
	const START: u32 = line!();
	fn ensure_location(location: &Location) {
		assert!(location.file().ends_with("tests.rs"));
		assert!(location.line() > START && location.line() < END);
	}

	let error = CtxError::new("test").context("test");
	error.contexts().map(|ctx| ctx.location).for_each(ensure_location);

	let src = "".parse::<bool>().unwrap_err();
	let result: Result<()> =
		Err(CtxError::new_with_source("test", src)).context("test").context_with(|| "test");
	result.unwrap_err().contexts().map(|ctx| ctx.location).for_each(ensure_location);

	let result: Result<bool> = source().context("test");
	result.unwrap_err().contexts().map(|ctx| ctx.location).for_each(ensure_location);

	let result: Result<bool> = source().context_with(|_| "test");
	result.unwrap_err().contexts().map(|ctx| ctx.location).for_each(ensure_location);

	#[expect(clippy::items_after_statements, reason = "We need the line number of the end")]
	const END: u32 = line!();
}

#[cfg(feature = "std")]
#[test]
fn exit_code() {
	use std::process::{ExitCode, Termination};

	let error = CtxError::new("test");
	assert_eq!(Termination::report(error), ExitCode::FAILURE);

	let error = CtxError::new("test").attach(ExitCode::SUCCESS);
	assert_eq!(Termination::report(error), ExitCode::SUCCESS);
}

#[test]
fn attach_override() {
	let error =
		CtxError::new("test").attach_override(false).attach_override('c').attach_override(true);
	assert!(*error.attachment::<bool>().unwrap());
	assert_eq!(error.attachments::<bool>().count(), 1);
}

#[test]
fn attach() {
	let error = CtxError::new("test").attach(false).attach('c').attach(true);
	assert!(*error.attachment::<bool>().unwrap());
	assert_eq!(error.attachments::<bool>().count(), 2);
}

#[test]
fn multi_errors() {
	let mut errors: Vec<CtxError> = Vec::new();
	level1().or_collect(&mut errors);
	level2().or_collect(&mut errors);
	assert_eq!(errors.len(), 2);
}

#[cfg(all(not(feature = "send"), not(feature = "sync")))]
#[test]
fn no_send_sync() {
	use ::core::marker::PhantomData;

	#[derive(Debug)]
	struct Source(PhantomData<*mut ()>); // Neither Send nor Sync.
	impl Display for Source {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			f.write_str("Source")
		}
	}
	impl Error for Source {}

	_ = CtxError::from_source(Source(PhantomData));
}

#[cfg(all(feature = "send", not(feature = "sync")))]
#[test]
fn send_not_sync() {
	use ::core::{cell::Cell, marker::PhantomData};

	#[derive(Debug)]
	struct Source(PhantomData<Cell<()>>); // Send, but not Sync.
	impl Display for Source {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			f.write_str("Source")
		}
	}
	impl Error for Source {}

	_ = CtxError::from_source(Source(PhantomData));
}

#[cfg(all(feature = "send", feature = "sync"))]
#[test]
fn send_sync() {
	use ::core::marker::PhantomData;

	#[derive(Debug)]
	struct Source(PhantomData<()>); // Send and Sync.
	impl Display for Source {
		fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
			f.write_str("Source")
		}
	}
	impl Error for Source {}

	_ = CtxError::from_source(Source(PhantomData));
}


#[derive(Debug)]
struct SourceError(core::str::ParseBoolError);

impl Display for SourceError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("SourceError occurred")
	}
}

impl Error for SourceError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.0)
	}
}

impl From<core::str::ParseBoolError> for SourceError {
	fn from(value: core::str::ParseBoolError) -> Self {
		Self(value)
	}
}


fn source() -> Result<bool, core::str::ParseBoolError> {
	"wahr".parse::<bool>()
}

fn source_source() -> Result<(), SourceError> {
	source()?;
	Result::Ok(())
}

fn level0() -> Result<()> {
	source_source().context("Level 0 error")
}

fn level1() -> Result<()> {
	level0().context("Level 1 error")
}

fn level2() -> Result<()> {
	level1().context("Level 2 error")
}
