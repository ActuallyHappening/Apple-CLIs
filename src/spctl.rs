use crate::prelude::*;

pub mod assess;

pub struct SpctlCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(SpctlCLIInstance, "spctl");