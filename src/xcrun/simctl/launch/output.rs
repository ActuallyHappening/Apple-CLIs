use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/must_use_command_output.md"))]
pub enum LaunchOutput {
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_error.md"))]
	ErrorUnImplemented(String),

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_success.md"))]
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
