use camino::{Utf8Path, Utf8PathBuf};

pub mod find_certificate;
pub use find_certificate::Certificate;

use crate::{impl_exec_instance, shared::CreateInstanceError};

pub struct SecurityCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(SecurityCLIInstance);