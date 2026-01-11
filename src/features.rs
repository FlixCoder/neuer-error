//! Feature selection configuration of types.

use ::core::{any::Any, error::Error, fmt::Debug};


/// Send trait, if feature is enabled, otherwise nothing.
#[cfg(feature = "send")]
pub trait PotentiallySend: Send {}
#[cfg(feature = "send")]
impl<T: Send> PotentiallySend for T {}
/// Send trait, if feature is enabled, otherwise nothing.
#[cfg(not(feature = "send"))]
pub trait PotentiallySend {}
#[cfg(not(feature = "send"))]
impl<T> PotentiallySend for T {}

/// Sync trait, if feature is enabled, otherwise nothing.
#[cfg(feature = "sync")]
pub trait PotentiallySync: Sync {}
#[cfg(feature = "sync")]
impl<T: Sync> PotentiallySync for T {}
/// Sync trait, if feature is enabled, otherwise nothing.
#[cfg(not(feature = "sync"))]
pub trait PotentiallySync {}
#[cfg(not(feature = "sync"))]
impl<T> PotentiallySync for T {}

/// Activated Send / Sync traits, if enabled.
#[diagnostic::on_unimplemented(
	message = "Make sure your type implements Send/Sync according to the activated crate features"
)]
pub trait SendSync: PotentiallySend + PotentiallySync {}
impl<T: PotentiallySend + PotentiallySync> SendSync for T {}

/// Any + Debug traits with send/sync.
#[diagnostic::on_unimplemented(
	message = "Make sure your type implements Debug and Send/Sync according to the activated crate features"
)]
pub trait AnyDebugSendSync: Any + Debug + SendSync {}
impl<T: Any + Debug + SendSync> AnyDebugSendSync for T {}

/// Error trait with send/sync.
pub trait ErrorSendSync: Error + SendSync {}
impl<T: Error + SendSync> ErrorSendSync for T {}
