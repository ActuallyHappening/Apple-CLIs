use crate::{impl_exec_instance, prelude::*};

pub mod well_known;

pub struct OpenCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(OpenCLIInstance, "open");

impl OpenCLIInstance {
	#[tracing::instrument(level = "trace", skip(self, path))]
	pub fn open_path(&self, path: impl AsRef<Utf8Path>) -> Result<bossy::ExitStatus> {
		Ok(
			self
				.bossy_command()
				.with_arg(path.as_ref())
				.run_and_wait()?,
		)
	}
}
