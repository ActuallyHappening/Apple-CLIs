use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum LaunchOutput {
	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented(String),

	#[doc = include_doc!(cmd_success)]
	SuccessUnImplemented(String),
}

impl CommandNomParsable for LaunchOutput {
	fn success_unimplemented(str: String) -> Self {
		Self::SuccessUnImplemented(str)
	}

	fn error_unimplemented(str: String) -> Self {
		Self::ErrorUnImplemented(str)
	}
}

impl PublicCommandOutput for LaunchOutput {
	type PrimarySuccess = ();

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			LaunchOutput::SuccessUnImplemented(_) => Ok(&()),
			LaunchOutput::ErrorUnImplemented(_) => Err(Error::output_errored(self)),
		}
	}
}
