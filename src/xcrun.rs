use camino::{Utf8Path, Utf8PathBuf};

use crate::{impl_exec_instance, shared::ExecInstance};

pub mod simctl;

#[derive(Debug)]
pub struct XcRunInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(XcRunInstance);