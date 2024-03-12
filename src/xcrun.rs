use camino::{Utf8Path, Utf8PathBuf};

use crate::shared::ExecInstance;

pub mod simctl;

#[derive(Debug)]
pub struct XcRunInstance {
	exec_path: Utf8PathBuf,
}

impl ExecInstance for XcRunInstance {
	const BINARY_NAME: &'static str = "xcrun";

	unsafe fn new_from_exec_path(exec_path: impl AsRef<Utf8Path>) -> Self {
		XcRunInstance {
			exec_path: exec_path.as_ref().to_path_buf(),
		}
	}

	fn get_inner_exec_path(&self) -> &Utf8Path {
		&self.exec_path
	}
}
