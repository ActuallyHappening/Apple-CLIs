use super::error::Error;
use crate::prelude::*;

pub use self::signed_keys::SignedKeys;
mod signed_keys;

#[derive(Debug, Serialize)]
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
	UnImplemented(String),
}

impl_from_str_nom!(DisplayOutput);

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

impl NomFromStr for DisplayOutput {
	fn nom_from_str(input: &str) -> nom::IResult<&str, Self> {
		alt((
			parse_key_value,
			map_res(rest, |s| {
				SignedKeys::from_raw(s).map(DisplayOutput::SignedKeys)
			}),
			map(ws(rest), |s: &str| {
				debug!(?s, "Parsed SuccessUnimplemented");
				DisplayOutput::UnImplemented(s.to_owned())
			}),
		))(input)
	}
}
