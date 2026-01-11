//! Macros for the users.

/// Create a helper trait `CtxErrorAttachments` that is implemented for
/// [`CtxError`](crate::CtxError), which allows to directly retrieve your attachments. You can
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
/// This will create a method `fn retryable(&self) -> Option<&Retryable>` on `CtxError`.
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
/// This will create a method `fn retryable(&self) -> Retryable` on `CtxError`. The closure receives
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
/// This will create a method `fn user_info(&self) -> String` on `CtxError`, which collects all
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
		#[doc = "Helper trait that is implemented for [`CtxError`], which allows to comfortably retrieve typed context information."]
		pub trait CtxErrorAttachments {
			$(
				$crate::provided_attachments!(@declare $getter_name($multiplicity_matcher: $attachment_type) -> $return_type {
					|$bind| $transform
				});
			)*
		}

		impl CtxErrorAttachments for $crate::CtxError {
			$(
				$crate::provided_attachments!(@implement $getter_name($multiplicity_matcher: $attachment_type) -> $return_type {
					|$bind| $transform
				});
			)*
		}
	};
}
