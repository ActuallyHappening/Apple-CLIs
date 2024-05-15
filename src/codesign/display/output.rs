use crate::prelude::*;

pub use self::signed_keys::SignedKeys;
mod signed_keys;

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum DisplayOutput {
	/// Considered an error case
	NotSignedAtAll {
		path: Utf8PathBuf,
	},

	/// Successfully extracted key-value pairs from codesign -d
	SignedKeys(signed_keys::SignedKeys),

	#[doc = include_doc!(cmd_success)]
	SuccessUnimplemented {
		stdout: String,
	},

	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented {
		stderr: String,
	},
}

impl CommandNomParsable for DisplayOutput {
	fn success_unimplemented(str: String) -> Self {
		Self::SuccessUnimplemented { stdout: str }
	}

	fn error_unimplemented(str: String) -> Self {
		Self::ErrorUnImplemented { stderr: str }
	}

	fn success_nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			parse_key_value,
			map_res(rest, |s| {
				SignedKeys::from_raw(s).map(DisplayOutput::SignedKeys)
			}),
		))(input)
	}
}

impl PublicCommandOutput for DisplayOutput {
	type PrimarySuccess = signed_keys::SignedKeys;

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			DisplayOutput::SignedKeys(keys) => Ok(keys),
			_ => Err(Error::output_errored(self)),
		}
	}
}

fn parse_key_value(input: &str) -> IResult<&str, DisplayOutput> {
	let (remaining, path) = map(
		terminated(take_while(|c| c != ':'), tag(": ")),
		Utf8Path::new,
	)(input)?;
	map(ws(tag("code object is not signed at all")), move |_| {
		debug!("Parsed NotSignedAtAll");
		DisplayOutput::NotSignedAtAll {
			path: path.to_owned(),
		}
	})(remaining)
}
