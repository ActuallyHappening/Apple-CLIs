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

	/// Added so a special recommendation of running a fixing command can be displayed:
	/// ```sh
	/// sudo rm -rf ~/Library/Developer/CoreSimulator/Caches
	/// ```
	/// 
	/// An error was encountered processing the command (domain=NSPOSIXErrorDomain, code=60):
	/// Unable to boot the Simulator.
	/// launchd failed to respond.
	/// Underlying error (domain=com.apple.SimLaunchHostService.RequestError, code=4):
	///         Failed to start launchd_sim: could not bind to session, launchd_sim may have crashed or quit responding
	ErrorLaunchDFailed {
		stderr: String,
	}
}

impl CommandNomParsable for BootOutput {
	fn success_unimplemented(stdout: String) -> Self {
		Self::SuccessUnImplemented { stdout }
	}

	fn error_unimplemented(stderr: String) -> Self {
		Self::ErrorUnImplemented { stderr }
	}

	fn errored_nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((parse_already_booted, parse_launchd_failed))(input)
	}
}

impl PublicCommandOutput for BootOutput {
	/// If successful, the simulator is booted successfully
	type PrimarySuccess = ();

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			BootOutput::SuccessUnImplemented { .. } | BootOutput::AlreadyBooted => Ok(&()),
			BootOutput::ErrorLaunchDFailed { .. } => {
				Err(Error::output_errored_with_hint(self, "Try running `sudo rm -rf ~/Library/Developer/CoreSimulator/Caches` to fix this issue."))
			}
			BootOutput::ErrorUnImplemented { .. } => Err(Error::output_errored(self)),
		}
	}
}

/// Parses [BootOutput::AlreadyBooted]
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

/// Parses [BootOutput::ErrorLaunchDFailed]
/// An error was encountered processing the command (domain=NSPOSIXErrorDomain, code=60):
/// Unable to boot the Simulator.
/// launchd failed to respond.
fn parse_launchd_failed(input: &str) -> IResult<&str, BootOutput> {
	let (remaining, _preamble) = ws(tag("An error was encountered processing the command"))(input)?;
	let (remaining, domain) =
		delimited(tag("(domain="), take_till(|c| c == ','), tag(","))(remaining)?;
	let (remaining, error_code) = delimited(ws(tag("code=")), digit1, ws(tag("):")))(remaining)?;
	let (underlying_error, msg) = ws(tag("launchd failed to respond."))(remaining)?;

	warn!(?domain, ?error_code, ?msg, ?underlying_error, "Parsed xcrun simctl boot error");

	Ok(("", BootOutput::ErrorLaunchDFailed { stderr: input.into() }))
}