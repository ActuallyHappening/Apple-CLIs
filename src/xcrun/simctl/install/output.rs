use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum InstallOutput {
	#[doc = include_doc!(cmd_success)]
	SuccessUnImplemented { stdout: String },

	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented { stderr: String },
}

impl CommandNomParsable for InstallOutput {
	fn success_unimplemented(stdout: String) -> Self {
		InstallOutput::SuccessUnImplemented { stdout }
	}

	fn error_unimplemented(stderr: String) -> Self {
		InstallOutput::ErrorUnImplemented { stderr }
	}
}

impl PublicCommandOutput for InstallOutput {
	type PrimarySuccess = ();

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			InstallOutput::SuccessUnImplemented { .. } => Ok(&()),
			InstallOutput::ErrorUnImplemented { .. } => Err(Error::output_errored(self)),
		}
	}
}
