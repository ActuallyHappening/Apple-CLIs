use std::num::NonZeroU8;

use nom::{character::complete::digit1, number::complete::float, IResult};
use serde::Deserialize;

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);

trait NomFromStr: Sized {
	fn nom_from_str(input: &str) -> IResult<&str, Self>;
}

impl NomFromStr for NonZeroU8 {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, number) = digit1(input)?;
		let number: NonZeroU8 = number.parse().map_err(|_err| {
			nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
		})?;
		Ok((remaining, number))
	}
}

pub mod device_name {
	use std::{num::NonZeroU8, str::FromStr};

	#[allow(unused_imports)]
	use nom::{
		branch::alt,
		bytes::complete::{tag, take_till, take_until},
		character::complete::{alpha0, alpha1, digit1, space0, space1},
		combinator::{success, value},
		sequence::{delimited, preceded, terminated},
		IResult,
	};
	use nom::{number::complete::float, sequence::tuple};
	use strum::EnumDiscriminants;
	use tracing::debug;

	use super::NomFromStr;

	#[derive(thiserror::Error, Debug)]
	pub enum DeviceNameParseError {
		#[error("Failed to parse device name")]
		ParsingFailed(#[source] nom::error::Error<String>),
	}

	#[derive(Debug)]
	pub enum DeviceName {
		IPhone(IPhoneVariant),

		Ipad {
			variant: IPadVariant,
			generation: Generation,
		},

		#[doc = include_str!("../../docs/TODO.md")]
		UnImplemented(String),
	}

	fn parse_iphone_discriminate(input: &str) -> IResult<&str, bool> {
		alt((value(true, tag("iPhone")), success(false)))(input)
	}

	#[derive(Debug)]
	pub enum IPhoneVariant {
		SE {
			generation: Generation,
		},

		Number {
			num: NonZeroU8,
			plus: bool,
			pro: bool,
			max: bool,
		},

		#[doc = include_str!("../../docs/TODO.md")]
		UnImplemented,
	}

	#[derive(Debug, Clone, Copy, PartialEq, EnumDiscriminants)]
	pub enum IPadVariant {
		Mini {
			generation: Generation,
		},
		Air {
			generation: Generation,
		},
		Plain {
			generation: Generation,
		},
		Pro {
			size: ScreenSize,
			generation: Generation,
		},
	}

	#[derive(Debug, Clone, Copy, PartialEq)]
	pub struct ScreenSize {
		inches: f32,
	}

	impl ScreenSize {
		fn new(inches: f32) -> Self {
			Self { inches }
		}
	}

	impl NomFromStr for ScreenSize {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
			let (remaining, inches) = delimited(tag("("), float, tag("-inch)"))(input)?;
			Ok((remaining, ScreenSize::new(inches)))
		}
	}

	impl NomFromStr for IPadVariant {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
			let (remaining, discriminate) = alt((
				value(IPadVariantDiscriminants::Mini, tag("mini")),
				value(IPadVariantDiscriminants::Air, tag("Air")),
				value(IPadVariantDiscriminants::Pro, tag("Pro")),
				success(IPadVariantDiscriminants::Plain),
			))(input)?;

			match discriminate {
				IPadVariantDiscriminants::Air => {
					let (remaining, generation) = Generation::nom_from_str(remaining)?;
					Ok((remaining, IPadVariant::Air { generation }))
				}
				IPadVariantDiscriminants::Mini => {
					let (remaining, generation) = Generation::nom_from_str(remaining)?;
					Ok((remaining, IPadVariant::Mini { generation }))
				}
				IPadVariantDiscriminants::Plain => {
					let (remaining, generation) = Generation::nom_from_str(remaining)?;
					Ok((remaining, IPadVariant::Plain { generation }))
				}
				IPadVariantDiscriminants::Pro => {
					let (remaining, size) = ScreenSize::nom_from_str(remaining)?;
					let (remaining, generation) = Generation::nom_from_str(remaining)?;
					Ok((remaining, IPadVariant::Pro { size, generation }))
				}
			}
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct Generation(NonZeroU8);

	fn ordinal(input: &str) -> IResult<&str, &str> {
		alt((tag("st"), tag("nd"), tag("rd"), tag("th")))(input)
	}

	impl NomFromStr for Generation {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
			let (remaining, number) = delimited(tag("("), NonZeroU8::nom_from_str, ordinal)(input)?;
			let (remaining, _) = tag("generation)")(remaining)?; // consume the closing parenthesis

			Ok((remaining, Generation(number)))
		}
	}

	impl FromStr for DeviceName {
		type Err = DeviceNameParseError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let mut input = String::from(s);
			input.retain(char::is_whitespace);
			let input: &str = &input;

			let (_remaining, device) = parse_device_name(input).map_err(|e| {
				debug!("Failed to parse device name: {:?}", e);
				// DeviceNameParseError::ParsingFailed(e.to_owned())
				match e.to_owned() {
					nom::Err::Error(e) | nom::Err::Failure(e) => DeviceNameParseError::ParsingFailed(e),
					nom::Err::Incomplete(_e) => unreachable!(),
				}
			})?;

			Ok(device)
		}
	}

	#[cfg(test)]
	mod tests {
		use tracing::debug;

		use super::*;

		#[test]
		fn test_parse_ordinal() {
			let examples = ["st", "nd", "th"];
			for example in examples.iter() {
				let output = ordinal(example);
				match output {
					Ok((remaining, _)) => {
						debug!("Parsed ordinal from {}: {:?}", example, remaining)
					}
					Err(e) => panic!("Failed to parse {:?}: {}", example, e),
				}
			}
		}

		#[test]
		fn test_parse_generation() {
			let examples = [
				"(1st generation)",
				"(2nd generation)",
				"(3rd generation)",
				"(4th generation)",
			];
			for example in examples.iter() {
				let output = parse_generation(example);
				match output {
					Ok((remaining, generation)) => {
						debug!(
							"Parsed generation: {:?} from {} [remaining: {}]",
							generation, example, remaining
						);
						assert!(
							remaining.is_empty(),
							"Remaining was not empty: {}",
							remaining
						);
					}
					Err(e) => panic!("Failed to parse {:?}: {}", example, e),
				}
			}
		}

		#[test]
		fn test_parse_device_name() {
			let examples = include!("../../tests/names.json");
			for example in examples.iter() {
				let output = parse_device_name(example);
				match output {
					Ok((remaining, device)) => {
						debug!(
							"Parsed device: {:?} from {} [remaining: {}]",
							device, example, remaining
						);
						assert!(
							remaining.is_empty(),
							"Remaining was not empty: {:?} (already parsed {:?})",
							remaining,
							device
						);
					}
					Err(e) => panic!("Failed to parse {:?}: {}", example, e),
				}
			}
		}
	}
}
