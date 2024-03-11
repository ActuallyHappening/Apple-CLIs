use camino::{Utf8Path, Utf8PathBuf};

pub mod detect;
pub mod upload;

#[derive(Debug)]
pub struct IosDeployInstance {
	exec_path: Utf8PathBuf,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateInstanceError {
	#[error("Error running `which ios-deploy`: {0}")]
	CommandExecution(#[from] which::Error),

	#[error("Error converting path to UTF-8: {0}")]
	PathNotUtf8(#[from] camino::FromPathBufError),
}

impl IosDeployInstance {
	pub fn new(exec_path: impl AsRef<Utf8Path>) -> IosDeployInstance {
		IosDeployInstance {
			exec_path: exec_path.as_ref().to_path_buf(),
		}
	}

	pub fn try_new_from_which() -> Result<IosDeployInstance, CreateInstanceError> {
		let path = which::which("ios-deploy")?;
		Ok(IosDeployInstance::new(Utf8PathBuf::try_from(path)?))
	}

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(&self.exec_path)
	}
}
