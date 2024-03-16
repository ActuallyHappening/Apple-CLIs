use crate::prelude::*;

pub mod find_certificate;
pub use find_certificate::Certificate;

#[derive(Debug)]
pub struct SecurityCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(SecurityCLIInstance, "security", skip_version_check);
