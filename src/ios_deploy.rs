use camino::{Utf8Path, Utf8PathBuf};

use crate::CreateInstanceError;

pub mod detect;
pub mod upload;

#[derive(Debug)]
pub struct IosDeployCLIInstance {
	exec_path: Utf8PathBuf,
}

impl IosDeployCLIInstance {
	pub fn new(exec_path: impl AsRef<Utf8Path>) -> IosDeployCLIInstance {
		IosDeployCLIInstance {
			exec_path: exec_path.as_ref().to_path_buf(),
		}
	}

	pub fn try_new_from_which() -> Result<IosDeployCLIInstance, CreateInstanceError> {
		let path = which::which("ios-deploy")?;
		Ok(IosDeployCLIInstance::new(Utf8PathBuf::try_from(path)?))
	}

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(&self.exec_path)
	}
}
