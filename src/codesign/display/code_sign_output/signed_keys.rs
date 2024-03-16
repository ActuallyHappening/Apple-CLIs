use std::{borrow::Cow, str::FromStr};

use nom::{
	bytes::complete::*,
	character::complete::{multispace0, newline},
	combinator::*,
	multi::fold_many1,
	sequence::*,
	IResult,
};
use time::macros::format_description;

use crate::prelude::*;

/// This will not parse some multi-key value things
/// e.g. "Sealed Resources version=2 rules=10 files=0"
/// becomes => "Sealed Resources version": "2 rules=10 files=0"
#[instrument(level = "trace")]
pub(super) fn parse_display_output(input: &str) -> IResult<&str, HashMap<Cow<str>, &str>> {
	let parse_key_value = pair(
		terminated(take_till1(|c| c == '='), tag("=")),
		terminated(take_till1(|c| c == '\n'), multispace0),
	);
	let (_, result) = all_consuming(fold_many1(
		parse_key_value,
		HashMap::<Cow<str>, &str>::new,
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
			acc.insert(key, value);
			acc
		},
	))(input)?;

	Ok(("", result))
}

#[derive(Debug, Serialize)]
pub struct SignedKeys {
	authority_1: String,
	executable: Utf8PathBuf,
	identifier: String,
	signed_time: time::PrimitiveDateTime,

	/// Includes the parsed keys above as well
	raw: HashMap<String, String>,
}

impl FromStr for SignedKeys {
	type Err = error::Error;

	fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
		debug!(?s, "Parsing SignedKeys from string");
		match parse_display_output(s) {
			Ok((_, result)) => Self::from_parsed(result),
			Err(err) => {
				debug!(?err, "Failed to parse SignedKeys from string");
				Err(Error::ParsingFailed {
					name: "SignedKeys".to_owned(),
					err: err.to_owned(),
				})
			}
		}
	}
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

	/// Even include the already parsed keys like [Self::identifier]
	pub fn raw(&self) -> HashMap<&str, &str> {
		HashMap::from_iter(self.raw.iter().map(|(k, v)| (k.as_str(), v.as_str())))
	}

	#[instrument(level = "trace", skip(input))]
	pub(crate) fn from_raw(input: &str) -> error::Result<Self> {
		input.parse()
	}

	const DATE_FORMAT: &'static [time::format_description::FormatItem<'static>] =
		format_description!(version = 2, "[day] [month repr:short] [year] at [hour padding:none]:[minute]:[second] [period case_sensitive:false]");

	#[instrument(level = "trace", skip(raw), ret)]
	fn from_parsed(raw: HashMap<Cow<str>, &str>) -> error::Result<Self> {
		debug!(?raw, "Extracting SignedKeys from parsed input");
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
				&raw
					.get("Signed Time")
					.ok_or_else(|| error::Error::SigningPropertyNotFound {
						missing_key: "Signed Time".into(),
					})?
					.to_string()
					.replace(' ', " "), // replace stupid space with good space to [time::PrimitiveDateTime::parse] works
				&Self::DATE_FORMAT,
			)?,
			raw: raw
				.into_iter()
				.map(|(k, v)| (k.into_owned(), v.to_string()))
				.collect(),
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse_raw_display_output() {
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

	#[test]
	fn test_basic_times_parse() {
		let fmt =
			format_description!("[hour padding:none]:[minute]:[second] [period case_sensitive:false]");
		let examples = ["2:41:28 pm"];
		for example in examples {
			match time::Time::parse(example, &fmt) {
				Ok(result) => {
					println!("Parsed: {:#?}", result);
				}
				Err(err) => {
					panic!("Failed to parse {}: {:?}", example, err);
				}
			}
		}

		let fmt = format_description!(
			" at [hour padding:none]:[minute]:[second] [period case_sensitive:false]"
		);
		let examples = [" at 2:41:28 pm"];
		for example in examples {
			match time::Time::parse(example, &fmt) {
				Ok(result) => {
					println!("Parsed: {:#?}", result);
				}
				Err(err) => {
					panic!("Failed to parse {}: {:?}", example, err);
				}
			}
		}
	}

	#[test]
	fn test_basic_dates_parse() {
		let fmt = format_description!(version = 2, "[day] [month repr:short] [year] at [hour padding:none]:[minute]:[second] [period case_sensitive:false]");
		let examples = ["16 Mar 2024 at 2:41:28 pm"];
		for example in examples {
			match time::PrimitiveDateTime::parse(example, &fmt) {
				Ok(result) => {
					println!("Parsed: {:#?}", result);
				}
				Err(err) => {
					panic!("Failed to parse {}: {:?}", example, err);
				}
			}
		}
	}

	#[test]
	fn test_date_parses() {
		println!(
			"this took me an hour to debug: char: {} and {}",
			' ' as u32, ' ' as u32
		);
		let fmt = SignedKeys::DATE_FORMAT;
		let examples = ["16 Mar 2024 at 2:41:28 pm"];
		for example in examples {
			match time::PrimitiveDateTime::parse(example, &fmt) {
				Ok(result) => {
					println!("Parsed: {:#?}", result);
				}
				Err(err) => {
					panic!("Failed to parse {}: {:?}", example, err);
				}
			}
		}
	}

	#[test]
	fn test_parse_extracted_display_output() {
		let test_input = include_str!(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/tests/codesign-display.txt"
		));
		match SignedKeys::from_str(test_input) {
			Ok(result) => {
				println!("Parsed: {:#?}", result);
			}
			Err(err) => {
				panic!("Failed to parse: {:?}", err);
			}
		}
	}
}
