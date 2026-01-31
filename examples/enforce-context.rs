//! How to force yourself to give context.
#![allow(clippy::missing_docs_in_private_items, reason = "Example")]

use ::macro_rules_attribute::apply;
use ::neuer_error::{Result, require_context, traits::*};

fn some_error() -> Result<bool, ::core::str::ParseBoolError> {
	"".parse::<bool>()
}

fn no_context_required() -> Result<()> {
	some_error()?;
	Ok(())
}

#[apply(require_context)]
fn context_required() -> Result<()> {
	some_error().context("compile error without this context")?;
	Ok(())
}

#[apply(require_context)]
fn main() -> Result<()> {
	no_context_required().context("compile error without this context")?;
	context_required().context("compile error without this context")?;
	Ok(())
}
