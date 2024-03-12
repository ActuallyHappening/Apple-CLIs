use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

pub mod identifiers;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	pub device_identifier: String,
	pub device_name: String,
	pub model_name: String,
	pub interface: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateInstanceError {
	#[error("Error running `which ios-deploy`: {0}")]
	CommandExecution(#[from] which::Error),

	#[error("Error converting path to UTF-8: {0}")]
	PathNotUtf8(#[from] camino::FromPathBufError),
}

/// Wrapper of binary
pub trait ExecInstance: Sized {
	/// E.g. `codesign` or `xcrun`
	const BINARY_NAME: &'static str;

	/// # Safety
	/// Must point to a valid executable file
	unsafe fn new_from_exec_path(exec_path: impl AsRef<Utf8Path>) -> Self;

	fn get_inner_exec_path(&self) -> &Utf8Path;

	fn try_new_from_which() -> Result<Self, CreateInstanceError> {
		let path = which::which(Self::BINARY_NAME)?;
		let path = Utf8PathBuf::try_from(path)?;
		let instance = unsafe { Self::new_from_exec_path(path) };
		Ok(instance)
	}
}

/// Wrapper of any command, including subcommands
pub trait ExecCommand: Sized {
	const COMMAND_SUFFIX: &'static str;

	fn bossy_command(&self) -> bossy::Command;
}

impl<T: ExecInstance> ExecCommand for T {
	/// e.g. `codesign` or `simctl` (not `xcrun simctl`)
	const COMMAND_SUFFIX: &'static str = T::BINARY_NAME;

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(self.get_inner_exec_path())
	}
}
