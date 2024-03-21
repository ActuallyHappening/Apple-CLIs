use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/must_use_command_output.md"))]
pub enum InstallOutput {
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_success.md"))]
	SuccessUnImplemented { stdout: String },

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_error.md"))]
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
