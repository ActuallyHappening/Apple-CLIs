use camino::{Utf8Path, Utf8PathBuf};

use crate::{impl_exec_instance, shared::CreateInstanceError};

mod assess;

pub struct SpctlCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(SpctlCLIInstance);