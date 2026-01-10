//! Validating data while collecting multiple errors.
#![allow(
	dead_code,
	clippy::missing_docs_in_private_items,
	clippy::print_stderr,
	reason = "Example"
)]

use ::contextual_errors::{CtxError, Result, traits::*};

struct UserData {
	id: u64,
	name: String,
	balance: i64,
}

fn validate_id(id: u64) -> Result<()> {
	if id == 0 { Err(CtxError::new("ID must be non-zero")) } else { Ok(()) }
}

fn validate_name(name: &str) -> Result<()> {
	if name.trim().is_empty() {
		Err(CtxError::new("Name must not be empty"))
	} else if !name.chars().all(|c| c.is_alphabetic()) {
		Err(CtxError::new("Name must only contain alphabetic characters"))
	} else {
		Ok(())
	}
}

struct User {
	id: u64,
	name: String,
	balance: i64,
}

impl User {
	fn new(data: UserData) -> Result<Self, Vec<CtxError>> {
		let mut errors = Vec::new();
		let UserData { id, name, balance } = data;

		validate_id(id).or_collect(&mut errors);
		validate_name(&name).or_collect(&mut errors);

		if balance < 0 {
			errors.push(CtxError::new("Cannot create new user with debt"));
		}

		if id == 3 {
			errors.push(CtxError::new(format!("User {id} ({name}) already exists")));
		}

		let user = User { id, name, balance };
		if errors.is_empty() { Ok(user) } else { Err(errors) }
	}
}

fn main() {
	match User::new(UserData { id: 1, name: "uwu".to_owned(), balance: 12345 }) {
		Ok(_user) => {
			eprintln!("User valid");
		}
		Err(errors) => {
			for error in errors {
				eprintln!("Error: {error}");
			}
		}
	}
}
