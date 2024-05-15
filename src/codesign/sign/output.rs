use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum SignOutput {
	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented { stderr: String },

	#[doc = include_doc!(cmd_success)]
	SuccessUnImplemented { stdout: String },
}

impl CommandNomParsable for SignOutput {
	fn success_unimplemented(stdout: String) -> Self {
		Self::SuccessUnImplemented { stdout }
	}

	fn error_unimplemented(stderr: String) -> Self {
		Self::ErrorUnImplemented { stderr }
	}
}

impl PublicCommandOutput for SignOutput {
	type PrimarySuccess = ();

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self.successful() {
			true => Ok(&()),
			false => Err(Error::output_errored(self))
		}
	}

	fn successful(&self) -> bool {
		matches!(self, Self::SuccessUnImplemented { .. })
	}
}