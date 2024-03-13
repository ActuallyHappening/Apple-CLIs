use crate::prelude::*;

pub struct OpenCLIInstance {
	exec_path: Utf8PathBuf,
}

impl ExecInstance for OpenCLIInstance {
	const BINARY_NAME: &'static str = "open";
	
	unsafe fn new_unchecked(exec_path: impl AsRef<Utf8Path>) -> Self {
		Self {
			exec_path: exec_path.as_ref().to_path_buf(),
		}
	}

	fn get_inner_exec_path(&self) -> &Utf8Path {
		&self.exec_path
	}
}