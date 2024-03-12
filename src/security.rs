use camino::{Utf8Path, Utf8PathBuf};

pub mod find_certificate;
pub use find_certificate::Certificate;

use crate::shared::CreateInstanceError;

pub struct SecurityCLIInstance {
	exec_path: Utf8PathBuf,
}

impl SecurityCLIInstance {
	pub fn new(exec_path: impl AsRef<Utf8Path>) -> SecurityCLIInstance {
		SecurityCLIInstance {
			exec_path: exec_path.as_ref().to_path_buf(),
		}
	}

	pub fn try_new_from_which() -> Result<SecurityCLIInstance, CreateInstanceError> {
		let path = which::which("security")?;
		Ok(SecurityCLIInstance::new(Utf8PathBuf::try_from(path)?))
	}

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(&self.exec_path)
	}
}
