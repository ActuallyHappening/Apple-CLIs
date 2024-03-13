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

	#[error("Calling `--version` failed: {0}")]
	VersionCheckFailed(#[from] bossy::Error),

	#[error("Path does not exist: {path} (std::io::Error: {err:?})")]
	PathDoesNotExist {
		path: Utf8PathBuf,
		err: Option<std::io::Error>,
	},
}

/// Wrapper of binary
pub trait ExecInstance: Sized {
	/// E.g. `codesign` or `xcrun`
	const BINARY_NAME: &'static str;

	/// # Safety
	/// Must point to a valid executable file.
	///
	/// Prefer [ExecInstance::new]
	unsafe fn new_unchecked(exec_path: impl AsRef<Utf8Path>) -> Self;

	fn get_inner_exec_path(&self) -> &Utf8Path;

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(self.get_inner_exec_path())
	}

	fn version_command(&self) -> bossy::Command {
		self.bossy_command().with_arg("--version")
	}

	fn new(path: impl AsRef<Utf8Path>) -> Result<Self, CreateInstanceError> {
		// check path exists
		let path = path.as_ref();
		match path.try_exists() {
			Ok(true) => Ok(unsafe { Self::new_unchecked(path) }),
			Ok(false) => Err(CreateInstanceError::PathDoesNotExist {
				path: path.to_owned(),
				err: None,
			}),
			Err(e) => Err(CreateInstanceError::PathDoesNotExist {
				path: path.to_owned(),
				err: Some(e),
			}),
		}
	}

	fn try_new_from_which() -> Result<Self, CreateInstanceError> {
		let path = which::which(Self::BINARY_NAME)?;
		let path = Utf8PathBuf::try_from(path)?;
		let instance = unsafe { Self::new_unchecked(path) };
		Ok(instance)
	}
}
