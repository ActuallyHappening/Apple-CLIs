use crate::prelude::*;

pub mod simctl;

#[derive(Debug)]
pub struct XcRunInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(XcRunInstance, "xcrun");