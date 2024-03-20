use crate::prelude::*;

pub use self::signed_keys::SignedKeys;
mod signed_keys;

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub enum DisplayOutput {
	/// Basically an error case
	NotSignedAtAll {
		path: Utf8PathBuf,
	},

	SignedKeys(signed_keys::SignedKeys),

	/// Represents a successful call to `codesign -d`
	///
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	SuccessUnimplemented {
		stdout: String,
	},

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	ErrorUnImplemented {
		stderr: String,
	},
}

impl DebugNamed for DisplayOutput {
	fn name() -> &'static str {
		"DisplayOutput"
	}
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
