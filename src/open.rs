use crate::prelude::*;

pub mod well_known;

pub struct OpenCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(OpenCLIInstance, "open", skip_version_check);
// impl ExecInstance for OpenCLIInstance {
// 	const BINARY_NAME: &'static str = "open";

// 	unsafe fn new_unchecked(exec_path: impl AsRef<Utf8Path>) -> Self {
// 		Self {
// 			exec_path: exec_path.as_ref().to_owned(),
// 		}
// 	}

// 	fn get_inner_exec_path(&self) -> &Utf8Path {
// 		&self.exec_path
// 	}

// 	fn validate_version(&self) -> bool {
// 		// builtin usually
// 		true
// 	}
// }

// impl OpenCLIInstance {
// 	pub fn new() -> Result<Self> {
// 		<Self as ExecInstance>::new()
// 	}
// }

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
