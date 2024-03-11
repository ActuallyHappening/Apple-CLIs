use camino::{Utf8Path, Utf8PathBuf};

use crate::CreateInstanceError;

pub mod display;

pub struct CodesignCLIInstance {
	exec_path: Utf8PathBuf,
}

impl CodesignCLIInstance {
	pub fn new(exec_path: impl AsRef<Utf8Path>) -> CodesignCLIInstance {
		CodesignCLIInstance {
			exec_path: exec_path.as_ref().to_path_buf(),
		}
	}

	pub fn try_new_from_which() -> Result<CodesignCLIInstance, CreateInstanceError> {
		let path = which::which("codesign")?;
		Ok(CodesignCLIInstance::new(Utf8PathBuf::try_from(path)?))
	}

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(&self.exec_path)
	}
}

