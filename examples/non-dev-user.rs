//! Typical error handling for non developer user errors in applications.
#![allow(
	clippy::missing_const_for_fn,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	reason = "Example"
)]

use ::neuer_error::{NeuErr, Result, provided_attachments, traits::*};

/// The usual error message contains code file locations, so it is not always suitable for UI
/// errors. We can simply attach user friendly messages to the errors and still keep debuggable
/// errors for logs.
#[derive(Debug)]
struct UserErrorMessage(String);

// Automatically create a helper to easily retrieve the attachment.
provided_attachments!(
	user_errors(multiple: UserErrorMessage) -> impl Iterator<Item = &String> { |msgs| msgs.map(|UserErrorMessage(msg)| msg) }
);

#[derive(Debug)]
struct MyUser;

fn fetch_user() -> Result<MyUser> {
	Err(NeuErr::new("Failed!").attach(UserErrorMessage("Invalid user ID 5".to_owned())))
}

fn save_user(_user: MyUser) -> Result<()> {
	Ok(())
}

fn change_user_name() -> Result<()> {
	let user = fetch_user().attach_with(|| UserErrorMessage("Could not find user".to_owned()))?;
	save_user(user).attach_with(|| UserErrorMessage("Could not save user info".to_owned()))?;
	Ok(())
}

fn main() {
	let result =
		change_user_name().attach_with(|| UserErrorMessage("Could not rename user".to_owned()));
	if let Err(err) = result {
		eprint!("User errors: ");
		for user_msg in err.user_errors() {
			eprintln!("{user_msg}");
		}
	}
}
