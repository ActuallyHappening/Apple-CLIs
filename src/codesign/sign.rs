use camino::Utf8Path;

use crate::security::Certificate;

use super::CodesignCLIInstance;

#[derive(thiserror::Error, Debug)]
pub enum CodeSignError {
	#[error("Error running `codesign -s`: {0}")]
	ExecuteError(bossy::Error),
}

impl CodesignCLIInstance {
	pub fn sign(&self, cert: &Certificate, path: impl AsRef<Utf8Path>) -> Result<(), CodeSignError> {
		todo!()
	}
}
