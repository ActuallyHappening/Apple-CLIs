use std::str::FromStr;

use crate::{
	prelude::*,
	shared::{ws, NomFromStr},
};

use camino::Utf8Path;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_till, take_while},
	combinator::{map, rest, success}, sequence::terminated,
};
use serde::Serialize;

use super::CodesignCLIInstance;

#[derive(Debug, Serialize)]
pub enum CodeSignOutput {
	NotSignedAtAll {
		path: Utf8PathBuf,
	},

	/// Represents a successful call to `codesign -d`
	///
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	SuccessUnimplemented {
		stdout: String,
	},

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	UnImplemented(String),
}

impl NomFromStr for CodeSignOutput {
	fn nom_from_str(input: &str) -> nom::IResult<&str, Self> {
		let (remaining, path) = map(terminated(take_while(|c| c != ':'), tag(": ")), Utf8Path::new)(input)?;
		alt((
			map(ws(tag("code object is not signed at all")), move |_| {
				CodeSignOutput::NotSignedAtAll {
					path: path.to_owned(),
				}
			}),
			map(ws(rest), |s: &str| CodeSignOutput::UnImplemented(s.to_owned())),
		))(remaining)
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
			Err(e) => Err(Error::ParsingFailed {
				name: "CodeSignOutput".to_owned(),
				err: e.to_owned(),
			}),
		}
	}
}

impl CodesignCLIInstance {
	#[tracing::instrument(level = "trace", skip(path))]
	pub fn display(&self, path: impl AsRef<Utf8Path>) -> Result<CodeSignOutput> {
		let output = self
			.bossy_command()
			.with_arg("-d")
			.with_arg(path.as_ref())
			.run_and_wait_for_output();

		match output {
			Ok(output) => {
				let stdout = String::from_utf8_lossy(output.stdout()).to_string();
				Ok(CodeSignOutput::UnImplemented(stdout))
			}
			Err(err) => {
				match err.output() {
					None => Err(err.into()),
					Some(output) => {
						// handling not signed case
						let stderr = String::from_utf8_lossy(output.stderr());
						CodeSignOutput::from_str(&stderr)
					}
				}
			}
		}
	}
}
