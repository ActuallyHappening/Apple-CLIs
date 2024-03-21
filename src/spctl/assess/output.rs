use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/must_use_command_output.md"))]
pub enum AssessOutput {
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_error.md"))]
	ErrorUnImplemented { stderr: String },

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_success.md"))]
	SuccessUnImplemented { stdout: String },
}

impl CommandNomParsable for AssessOutput {
	fn error_unimplemented(stderr: String) -> Self {
		Self::ErrorUnImplemented { stderr }
	}

	fn success_unimplemented(stdout: String) -> Self {
		Self::SuccessUnImplemented { stdout }
	}
}

impl PublicCommandOutput for AssessOutput {
	type PrimarySuccess = ();

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			AssessOutput::SuccessUnImplemented { .. } => Ok(&()),
			AssessOutput::ErrorUnImplemented { .. } => Err(Error::output_errored(self)),
		}
	}
}
