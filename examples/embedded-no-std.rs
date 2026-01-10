//! Typical error handling in no-std, no-alloc embedded environments.
#![no_std]
#![allow(
	dead_code,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	reason = "Example"
)]

use ::contextual_errors::{CtxError, Result, traits::*};

fn main() {}
