use crate::msg::WndMsg;

/// A [`Result` alias](crate#errors-and-result-aliases) for errors from window
/// message handling, which returns a [`MsgError`](crate::gui::MsgError) on
/// failure.
///
/// # Examples
///
/// Converting into the generic [`AnyResult`](crate::AnyResult):
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, AnyResult, gui::MsgResult};
///
/// let run_result: MsgResult<()> = Ok(());
///
/// let err_result: AnyResult<()> = run_result.map_err(|err| err.into());
/// ```
pub type MsgResult<T> = Result<T, MsgError>;

/// An error that occurred within a closure of a window message handling.
/// Usually these errors are thrown by the user closures.
///
/// This error types wraps the actual user error along with the parameters of
/// the message where the error happened.
///
/// The [`Result` alias](crate#errors-and-result-aliases) for this type is
/// [`MsgResult`](crate::gui::MsgResult).
pub struct MsgError {
	src_msg: WndMsg,
	source: Box<dyn std::error::Error + Send + Sync>,
}

impl std::error::Error for MsgError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		Some(self.source.as_ref())
	}
}

impl std::fmt::Debug for MsgError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "WM {} - {}",
			self.src_msg.msg_id, self.source.to_string())
	}
}

impl std::fmt::Display for MsgError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<Self as std::fmt::Debug>::fmt(self, f) // delegate to Debug trait
	}
}

impl MsgError {
	/// Constructs a new `MsgError` by wrapping the given error.
	#[must_use]
	pub const fn new(
		src_msg: WndMsg,
		source: Box<dyn std::error::Error + Send + Sync>) -> MsgError
	{
		Self { src_msg, source }
	}

	/// The source message information where the error originated from.
	#[must_use]
	pub const fn src_msg(&self) -> WndMsg {
		self.src_msg
	}
}
