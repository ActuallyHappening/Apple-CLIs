use crate::{impl_exec_instance, prelude::*};

pub mod well_known;

pub struct OpenCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(OpenCLIInstance, "open");

#[derive(thiserror::Error, Debug)]
pub enum OpenError {
	#[error("The command `open` failed: {0}")]
	ExecuteError(#[from] bossy::Error),

	#[error("The hard coded path {:?} was not found ({:?})", hard_coded_path, err)]
	WellKnownPathNotFound {
		hard_coded_path: Utf8PathBuf,
		err: Option<std::io::Error>,
	},
}

impl OpenCLIInstance {
	pub fn open_path(&self, path: impl AsRef<Utf8Path>) -> Result<bossy::ExitStatus, OpenError> {
		Ok(
			self
				.bossy_command()
				.with_arg(path.as_ref())
				.run_and_wait()?,
		)
	}
}