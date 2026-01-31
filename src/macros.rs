//! Macros for the users.

/// Create a helper trait `NeuErrAttachments` that is implemented for
/// [`NeuErr`](crate::NeuErr), which allows to directly retrieve your attachments. You can
/// modify visibility and name by re-exporting via `pub use` if needed.
///
/// This improves discoverability and allows you to unwrap potential new-types you might have had to
/// use (or wanted to use).
///
/// ## Usage
///
/// Simple getters without type transformation:
///
/// ```rust
/// # use neuer_error::provided_attachments;
/// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
/// enum Retryable { Yes, No }
///
/// provided_attachments!(
/// 	retryable(single: Retryable) -> Option<&Retryable> { |v| v };
/// );
/// ```
///
/// This will create a method `fn retryable(&self) -> Option<&Retryable>` on `NeuErr`.
///
/// You can also make use of the transformation expression that will be applied to the attachment
/// before returning it:
///
/// ```rust
/// # use neuer_error::provided_attachments;
/// #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
/// enum Retryable { Yes, No }
///
/// provided_attachments!(
/// 	retryable(single: Retryable) -> Retryable { |retry| retry.copied().unwrap_or(Retryable::No) };
/// );
/// ```
///
/// This will create a method `fn retryable(&self) -> Retryable` on `NeuErr`. The closure receives
/// the `Option<&Retryable>` and returns a `Retryable`.
///
/// Finally, you can also retrieve multiple attachments of the same type and transform the iterator
/// into your return type:
///
/// ```rust
/// # use neuer_error::provided_attachments;
/// #[derive(Debug, PartialEq, Clone)]
/// struct UserInfo(String);
///
/// provided_attachments!(
/// 	user_info(multiple: UserInfo) -> String { |iter| iter.map(|UserInfo(s)| s.as_str()).collect() };
/// );
/// ```
///
/// This will create a method `fn user_info(&self) -> String` on `NeuErr`, which collects all
/// `UserInfo` attachments, unpacks them and collects them into a single `String`.
#[macro_export]
macro_rules! provided_attachments {
	// Declare rule for single attachment.
	(@declare $getter_name:ident (single: $attachment_type:ty) -> $return_type:ty {
		// Transformation closure, receiving type Option<&$attachment_type> and returning $return_type.
		|$bind:ident| $transform:expr
	}) => {
		#[doc = concat!("Get attachment `", stringify!($getter_name), "` via type `", stringify!($attachment_type), "` from the error.")]
		fn $getter_name(&self) -> $return_type;
	};

	// Implement rule for single attachment.
	(@implement $getter_name:ident (single: $attachment_type:ty) -> $return_type:ty {
		// Transformation closure, receiving type Option<&$attachment_type> and returning $return_type.
		|$bind:ident| $transform:expr
	}) => {
		fn $getter_name(&self) -> $return_type {
			let $bind = Self::attachment::<$attachment_type>(self);
			$transform
		}
	};

	// Declare rule for multiple attachment.
	(@declare $getter_name:ident (multiple: $attachment_type:ty) -> $return_type:ty {
		// Transformation closure, receiving type impl Iterator<Item = &$attachment_type> and returning $return_type.
		|$bind:ident| $transform:expr
	}) => {
		#[doc = concat!("Get attachment `", stringify!($getter_name), "` via type `", stringify!($attachment_type), "` from the error.")]
		fn $getter_name(&self) -> $return_type;
	};

	// Implement rule for multiple attachment.
	(@implement $getter_name:ident (multiple: $attachment_type:ty) -> $return_type:ty {
		// Transformation closure, receiving type impl Iterator<Item = &$attachment_type> and returning $return_type.
		|$bind:ident| $transform:expr
	}) => {
		fn $getter_name(&self) -> $return_type {
			let $bind = Self::attachments::<$attachment_type>(self);
			$transform
		}
	};

	// Main matcher, splitting into attachment list.
	($(
		$getter_name:ident ($multiplicity_matcher:ident : $attachment_type:ty) -> $return_type:ty { |$bind:ident| $transform:expr }
	);* $(;)?) => {
		#[doc = "Helper trait that is implemented for [`NeuErr`], which allows to comfortably retrieve typed context information."]
		pub trait NeuErrAttachments {
			$(
				$crate::provided_attachments!(@declare $getter_name($multiplicity_matcher: $attachment_type) -> $return_type {
					|$bind| $transform
				});
			)*
		}

		impl NeuErrAttachments for $crate::NeuErr {
			$(
				$crate::provided_attachments!(@implement $getter_name($multiplicity_matcher: $attachment_type) -> $return_type {
					|$bind| $transform
				});
			)*
		}
	};
}

/// Apply this to a function, to automatically make inner errors require context.
/// Ensures there will be a compile-time error when you forget to provide context to an error within
/// the current function.
///
/// It is highly recommended to make use of the crate `macro_rules_attribute` to use it as an
/// attribute instead of surrounding the function with the macro:
///
/// ```
/// # use ::macro_rules_attribute::apply;
/// # use ::neuer_error::{Result, require_context};
/// #[apply(require_context)]
/// fn do_something() -> Result<()> {
/// 	Ok(())
/// }
/// ```
///
/// Otherwise, you can use it like this:
///
/// ```
/// # use ::neuer_error::{Result, require_context};
/// require_context!(
/// 	fn do_something() -> Result<()> {
/// 		Ok(())
/// 	}
/// );
/// ```
///
/// Only the following kinds of functions are supported currently:
///
/// - Function must not be const, unsafe or have an external ABI.
/// - Function cannot have generics.
///
/// It is also possible to use the macro within functions, specifying sync vs async with a marker
/// up-front:
///
/// ```
/// # use ::neuer_error::{Result, require_context};
/// fn do_something() -> Result<()> {
/// 	require_context!(@sync
/// 		let result = Ok(());
/// 		result
/// 	)
/// }
/// ```
#[macro_export]
macro_rules! require_context {
	// Macro entry for within async function body blocks.
	(@async $($body:tt)* ) => {{
		let call = async move || -> ::core::result::Result<_, $crate::NeuErr<$crate::ProvideContext>> { $($body)* };
		call().await.map_err($crate::NeuErr::<$crate::ProvideContext>::remove_marker)
	}};

	// Macro entry for within sync function body blocks.
	(@sync $($body:tt)* ) => {{
		let call = move || -> ::core::result::Result<_, $crate::NeuErr<$crate::ProvideContext>> { $($body)* };
		call().map_err($crate::NeuErr::<$crate::ProvideContext>::remove_marker)
	}};

	// Async functions.
	(
		$(#[$attrs:meta])*
		$vis:vis async fn $name:ident
		( $($params:tt)* ) -> $return:ty
		$body:block
	) => {
		$(#[$attrs])*
		$vis async fn $name
		( $($params)* ) -> $return
		{
			$crate::require_context!(@async $body)
		}
	};

	// Sync functions.
	(
		$(#[$attrs:meta])*
		$vis:vis fn $name:ident
		( $($params:tt)* ) -> $return:ty
		$body:block
	) => {
		$(#[$attrs])*
		$vis fn $name
		( $($params)* ) -> $return
		{
			$crate::require_context!(@sync $body)
		}
	};
}

#[cfg(test)]
mod compile_tests {
	#![allow(dead_code, reason = "Compile tests")]

	use ::macro_rules_attribute::apply;

	use crate::Result;

	require_context!(
		#[cfg(test)]
		pub async fn test1(#[cfg(test)] _param: bool, (): ()) -> Result<()> {
			Ok(())
		}
	);

	#[apply(require_context)]
	fn test2() -> Result<()> {
		Ok(())
	}
}
