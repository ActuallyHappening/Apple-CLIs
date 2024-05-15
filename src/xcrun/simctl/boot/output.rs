use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum BootOutput {
	/// NOT considered an error case, since the simulator is *already* booted.
	AlreadyBooted,

	#[doc = include_doc!(cmd_success)]
	SuccessUnImplemented {
		stdout: String,
	},

	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented {
		stderr: String,
	},
}

impl CommandNomParsable for BootOutput {
	fn success_unimplemented(stdout: String) -> Self {
		Self::SuccessUnImplemented { stdout }
	}

	fn error_unimplemented(stderr: String) -> Self {
		Self::ErrorUnImplemented { stderr }
	}

	fn errored_nom_from_str(input: &str) -> IResult<&str, Self> {
		parse_already_booted(input)
	}
}

impl PublicCommandOutput for BootOutput {
	/// If successful, the simulator is booted successfully
	type PrimarySuccess = ();

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			BootOutput::SuccessUnImplemented { .. } | BootOutput::AlreadyBooted => Ok(&()),
			BootOutput::ErrorUnImplemented { .. } => Err(Error::output_errored(self)),
		}
	}
}

fn parse_already_booted(input: &str) -> IResult<&str, BootOutput> {
	let (remaining, _preamble) = ws(tag("An error was encountered processing the command"))(input)?;
	let (remaining, domain) =
		delimited(tag("(domain="), take_till(|c| c == ','), tag(","))(remaining)?;
	let (remaining, error_code) = delimited(ws(tag("code=")), digit1, ws(tag("):")))(remaining)?;
	let (_, msg) =
		all_consuming(ws(tag("Unable to boot device in current state: Booted")))(remaining)?;

	warn!(?domain, ?error_code, ?msg, "Parsed xcrun simctl boot error");

	Ok(("", BootOutput::AlreadyBooted))
}