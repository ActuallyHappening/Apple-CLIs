use std::str::FromStr;

use crate::{
	prelude::*,
	shared::{ws, NomFromStr},
};

use camino::Utf8Path;
use nom::{
	branch::alt,
	bytes::complete::{tag, take_till, take_while},
	combinator::{map, rest, success},
	sequence::terminated,
};
use serde::Serialize;

use super::CodesignCLIInstance;

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

mod signed_keys {
	use std::borrow::Cow;

	use nom::{
		bytes::complete::*, character::complete::{multispace0, newline}, combinator::*, multi::fold_many1,
		sequence::*, IResult,
	};
	use time::macros::format_description;

	use crate::prelude::*;

	/// This will not parse some multi-key value things
	/// e.g. "Sealed Resources version=2 rules=10 files=0"
	/// becomes => "Sealed Resources version": "2 rules=10 files=0"
	pub(super) fn parse_display_output(input: &str) -> IResult<&str, HashMap<Cow<str>, String>> {
		let parse_key_value = pair(
			terminated(take_till1(|c| c == '='), tag("=")),
			terminated(take_till1(|c| c == '\n'), multispace0),
		);
		let (_, result) = all_consuming(fold_many1(
			parse_key_value,
			HashMap::<Cow<str>, String>::new,
			|mut acc: HashMap<_, _>, (key, value)| {
				let key = if key == "Authority" {
					let mut num = 1;
					let new_key: String = loop {
						let new_key = format!("Authority_{}", num);
						if !acc.contains_key(&Cow::<str>::Owned(new_key.clone())) {
							break new_key.clone();
						} else {
							num += 1;
						}
					};
					Cow::Owned(new_key)
				} else {
					Cow::Borrowed(key)
				};
				acc.insert(key, value.to_owned());
				acc
			},
		))(input)?;

		Ok(("", result))
	}

	#[test]
	fn test_parse_display_output() {
		let test_input = include_str!(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/tests/codesign-display.txt"
		));
		match parse_display_output(test_input) {
			Ok((_, result)) => {
				println!("Parsed: {:#?}", result);
			}
			Err(err) => {
				panic!("Failed to parse: {:?}", err);
			}
		}
	}

	#[derive(Debug, Serialize)]
	pub struct SignedKeys {
		authority_1: String,
		executable: Utf8PathBuf,
		identifier: String,
		signed_time: time::PrimitiveDateTime,
	}

	impl SignedKeys {
		pub fn authority_1(&self) -> &str {
			&self.authority_1
		}

		pub fn executable(&self) -> &Utf8PathBuf {
			&self.executable
		}

		pub fn identifier(&self) -> &str {
			&self.identifier
		}

		pub fn signed_time(&self) -> &time::PrimitiveDateTime {
			&self.signed_time
		}

		pub fn from_raw(raw: HashMap<Cow<str>, &str>) -> error::Result<Self> {
			let date_format =
				format_description!("[day] [month repr:short] [year] at [hour]:[minute]:[second] [period]");
			Ok(SignedKeys {
				authority_1: raw
					.get("Authority_1")
					.ok_or_else(|| error::Error::SigningPropertyNotFound {
						missing_key: "Authority".into(),
					})?
					.to_string(),
				executable: raw
					.get("Executable")
					.ok_or_else(|| error::Error::SigningPropertyNotFound {
						missing_key: "Executable".into(),
					})?
					.into(),
				identifier: raw
					.get("Identifier")
					.ok_or_else(|| error::Error::SigningPropertyNotFound {
						missing_key: "Identifier".into(),
					})?
					.to_string(),
				signed_time: time::PrimitiveDateTime::parse(
					raw
						.get("Signed Time")
						.ok_or_else(|| error::Error::SigningPropertyNotFound {
							missing_key: "Signed Time".into(),
						})?,
					&date_format,
				)?,
			})
		}
	}
}

impl NomFromStr for CodeSignOutput {
	fn nom_from_str(input: &str) -> nom::IResult<&str, Self> {
		let (remaining, path) = map(
			terminated(take_while(|c| c != ':'), tag(": ")),
			Utf8Path::new,
		)(input)?;
		alt((
			map(ws(tag("code object is not signed at all")), move |_| {
				CodeSignOutput::NotSignedAtAll {
					path: path.to_owned(),
				}
			}),
			map(ws(rest), |s: &str| {
				CodeSignOutput::UnImplemented(s.to_owned())
			}),
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
				let stderr = String::from_utf8_lossy(output.stderr()).to_string();
				debug!(%stdout, %stderr, "codesign outputted stdout");
				Ok(CodeSignOutput::UnImplemented(stderr))
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
