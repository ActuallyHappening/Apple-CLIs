use camino::{Utf8Path, Utf8PathBuf};

use crate::CreateInstanceError;

mod assess;

pub struct SpctlCLIInstance {
	exec_path: Utf8PathBuf
}

impl SpctlCLIInstance {
	pub fn new(exec_path: impl AsRef<Utf8Path>) -> SpctlCLIInstance {
		SpctlCLIInstance {
			exec_path: exec_path.as_ref().to_path_buf()
		}
	}

	pub fn try_new_from_which() -> Result<SpctlCLIInstance, CreateInstanceError> {
		let path = which::which("spctl")?;
		Ok(SpctlCLIInstance::new(Utf8PathBuf::try_from(path)?))
	}

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(&self.exec_path).with_arg("-vvvv")
	}
}