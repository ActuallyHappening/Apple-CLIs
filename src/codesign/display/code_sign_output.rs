use std::str::FromStr;

use camino::{Utf8Path, Utf8PathBuf};
use nom::{
	branch::alt,
	bytes::complete::{tag, take_while},
	combinator::{map, map_res, rest},
	sequence::terminated,
	IResult,
};
use serde::Serialize;
use tracing::debug;

use crate::{
	error,
	shared::{ws, NomFromStr},
};

use self::signed_keys::SignedKeys;

use super::error::Error;

mod signed_keys;

#[derive(Debug, Serialize)]
pub enum CodeSignOutput {
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

fn parse_key_value(input: &str) -> IResult<&str, CodeSignOutput> {
	let (remaining, path) = map(
		terminated(take_while(|c| c != ':'), tag(": ")),
		Utf8Path::new,
	)(input)?;
	map(ws(tag("code object is not signed at all")), move |_| {
		debug!("Parsed NotSignedAtAll");
		CodeSignOutput::NotSignedAtAll {
			path: path.to_owned(),
		}
	})(remaining)
}

impl NomFromStr for CodeSignOutput {
	fn nom_from_str(input: &str) -> nom::IResult<&str, Self> {
		alt((
			parse_key_value,
			map_res(rest, |s| {
				SignedKeys::from_raw(s).map(CodeSignOutput::SignedKeys)
			}),
			// map(ws(rest), |s: &str| {
			// 	debug!(?s, "Parsed SuccessUnimplemented");
			// 	CodeSignOutput::UnImplemented(s.to_owned())
			// }),
		))(input)
	}
}

impl FromStr for CodeSignOutput {
	type Err = error::Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		match CodeSignOutput::nom_from_str(s) {
			Ok(("", output)) => Ok(output),
			Ok((remaining, output)) => Err(Error::ParsingRemainingNotEmpty {
				input: s.to_owned(),
				remaining: remaining.to_owned(),
				parsed_debug: format!("{:#?}", output),
			}),
			Err(e) => Err(Error::NomParsingFailed {
				name: "CodeSignOutput".to_owned(),
				err: e.to_owned(),
			}),
		}
	}
}
