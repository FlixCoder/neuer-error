//! Typical error handling for developer CLI tools.
#![allow(
	dead_code,
	clippy::missing_docs_in_private_items,
	clippy::missing_const_for_fn,
	reason = "Example"
)]

use ::neuer_error::{CtxError, Result, traits::*};
use ::std::process::ExitCode;

fn ensure_project_validity() -> Result<()> {
	Ok(())
}

fn call_preprocessor() -> Result<()> {
	Err(CtxError::new("Binary gcc not found"))
}

fn compile_my_code() -> Result<()> {
	ensure_project_validity().context("project must be valid for compiling")?;
	call_preprocessor().context("preprocessor failed")?;
	Ok(())
}

fn lint() -> Result<()> {
	Err(CtxError::new("Warning: something is deprecated").attach_override(ExitCode::SUCCESS))
}

// Returning the error will automatically use the attached ExitCode or assume failure.
fn main() -> Result<()> {
	ensure_project_validity().context("Project is invalid")?;
	compile_my_code().context("Failed compiling code")?;
	lint()?;
	Ok(())
}
