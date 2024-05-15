use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum AssessOutput {
	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented { stderr: String },

	#[doc = include_doc!(cmd_success)]
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
