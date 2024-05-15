use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum UploadOutput {
	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented { stderr: String },

	#[doc = include_doc!(cmd_success)]
	SuccessUnImplemented { stdout: String },
}

impl CommandNomParsable for UploadOutput {
	fn error_unimplemented(stderr: String) -> Self {
		Self::ErrorUnImplemented { stderr }
	}

	fn success_unimplemented(stdout: String) -> Self {
		Self::SuccessUnImplemented { stdout }
	}
}

impl PublicCommandOutput for UploadOutput {
	type PrimarySuccess = ();

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			UploadOutput::SuccessUnImplemented { .. } => Ok(&()),
			UploadOutput::ErrorUnImplemented { .. } => Err(Error::output_errored(self)),
		}
	}
}
