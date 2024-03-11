use camino::Utf8Path;

use crate::security::Certificate;

use super::CodesignCLIInstance;

#[derive(thiserror::Error, Debug)]
pub enum CodeSignError {
	#[error("Error running `codesign -s`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

impl CodesignCLIInstance {
	pub fn sign(&self, cert: &Certificate, path: impl AsRef<Utf8Path>) -> Result<(), CodeSignError> {
		self
			.bossy_command()
			.with_arg("-s")
			.with_arg(&cert.common_name)
			.with_arg(path.as_ref())
			.run_and_wait_for_string()?;

		Ok(())
	}
}
