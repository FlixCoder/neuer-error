//! Typical web client error handling.
#![allow(
	dead_code,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	reason = "Example"
)]

use ::neuer_error::{CtxError, Result, provided_attachments, traits::*};
use ::std::time::Duration;

/// Mark errors	whether they can be retried and/or were already retried.
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

// Automatically create a helper to easily retrieve the attachment.
provided_attachments!(
	status(single: ErrorStatus) -> ErrorStatus { |status| status.copied().unwrap_or_default() }
);

/// Web client does some request. Returns some client-library error.
fn do_request(_request: String) -> Result<(), std::io::Error> {
	Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Interface eth1 not found"))
}

/// Your client method fetching some data from the server.
fn fetch_data(user: &str) -> Result<()> {
	let request = format!("https://test.test/users/{user}?authenticated=yes-trust-me");

	// Alternative 1.
	do_request(request.clone())
		.attach_override_with(|err| match err.kind() {
			std::io::ErrorKind::NetworkDown => ErrorStatus::Temporary,
			_ => ErrorStatus::Permanent,
		})
		.context_with(|| format!("failed fetching data for user {user}"))?;

	// Alternative 2.
	do_request(request).map_err(|err| {
		let status = match err.kind() {
			std::io::ErrorKind::NetworkDown => ErrorStatus::Temporary,
			_ => ErrorStatus::Permanent,
		};
		CtxError::new_with_source(format!("failed fetching data for user {user}"), err)
			.attach_override(status)
	})
}

fn main() {
	// Retry requests based on error status.
	loop {
		match fetch_data("alice") {
			Ok(data) => {
				eprintln!("Request successful, data: {data:?}");
				break;
			}
			Err(err) => {
				if err.status() == ErrorStatus::Temporary {
					eprintln!("Error: {err:#}; Retrying request in a bit..");
					std::thread::sleep(Duration::from_secs(2));
				} else {
					eprintln!("Permanent error fetching data: {err}");
					break;
				}
			}
		}
	}
}
