use crate::prelude::*;

#[derive(Debug, Serialize)]
pub enum LaunchOutput {
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	ErrorUnImplemented(String),

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
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
		match self.successful() {
			true => Ok(&()),
			false => Err(Error::output_errored(self)),
		}
	}

	fn successful(&self) -> bool {
		matches!(self, LaunchOutput::SuccessUnImplemented { .. })
	}
}
