//! Typical web server backend error handling.
#![allow(
	dead_code,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	reason = "Example"
)]

use ::neuer_error::{CtxError, Result, traits::*};

/// Wrapper to convert errors to HTTP responses automatically.
#[derive(Debug)]
struct ToResponse(CtxError);

impl IntoResponse for ToResponse {
	fn into_response(self) -> (StatusCode, String) {
		let status = self.0.attachment::<StatusCode>().copied().unwrap_or_default();
		let message = format!("{}", self.0); // Or maybe more "user-error-message" attachments.
		(status, message)
	}
}

impl From<CtxError> for ToResponse {
	fn from(err: CtxError) -> Self {
		Self(err)
	}
}

/// Request handler for a route.
///
/// The [`CtxError`] is automatically converted to our wrapper. At least if we gave context and use
/// the question mark operator.
fn handle_request(user: &str) -> Result<(), ToResponse> {
	match user {
		"" => {
			return Err(CtxError::new("User must not be empty")
				.attach(StatusCode::BadRequest)
				.into());
		}
		"alice" => manipulate().context("Failed manipulating")?,
		not_found => {
			return Err(CtxError::new(format!("User `{not_found}` was not found"))
				.attach(StatusCode::NotFound)
				.into());
		}
	}
	Ok(())
}

/// Certain action we want to do.
fn manipulate() -> Result<()> {
	unimplemented!()
}

fn main() {
	// Start up web server with handlers.
	// I just fake a single request here.
	let request = "bob";
	let (status, message) = handle_request(request).into_response();
	eprintln!("{status:?}: {message}");
}


/* Fake types and traits, similar to how they are found in axum and http. */

/// Fake axum trait `IntoResponse`, converting the type to an HTTP response.
trait IntoResponse {
	fn into_response(self) -> (StatusCode, String);
}

impl IntoResponse for () {
	fn into_response(self) -> (StatusCode, String) {
		(StatusCode::Ok, String::new())
	}
}

impl<T, E> IntoResponse for Result<T, E>
where
	T: IntoResponse,
	E: IntoResponse,
{
	fn into_response(self) -> (StatusCode, String) {
		match self {
			Ok(v) => v.into_response(),
			Err(e) => e.into_response(),
		}
	}
}

/// Fake HTTP status code from usual libraries.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
enum StatusCode {
	#[default]
	InternalError,
	BadRequest,
	NotFound,
	Ok,
}
