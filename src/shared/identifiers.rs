use serde::Deserialize;

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);

pub mod device_name {
	use std::{num::NonZeroU8, str::FromStr};

	use nom::{
		branch::alt,
		bytes::complete::{tag, take_till, take_until},
		character::complete::{alpha0, alpha1, digit1, space0, space1},
		combinator::value,
		sequence::{delimited, preceded},
		IResult,
	};
	use tracing::debug;

	#[derive(thiserror::Error, Debug)]
	pub enum DeviceNameParseError {
		#[error("Failed to parse device name")]
		ParsingFailed,
	}

	#[derive(Debug)]
	pub enum DeviceName {
		IPhone {
			variant: IPhoneVariant,
			plus: bool,
			pro: bool,
			max: bool,
		},
		#[doc = include_str!("../../docs/TODO.md")]
		UnImplemented(String),
	}

	fn parse_iphone_discriminate(input: &str) -> IResult<&str, bool> {
		value(true, tag("iPhone"))(input)
	}

	#[derive(Debug)]
	pub enum IPhoneVariant {
		SE { generation: Generation },
		Num(NonZeroU8),
	}

	#[derive(Debug)]
	pub struct Generation(NonZeroU8);

	fn ordinal(input: &str) -> IResult<&str, &str> {
		alt((tag("st"), tag("nd"), tag("rd"), tag("th")))(input)
	}

	fn non_zero_u8(input: &str) -> IResult<&str, NonZeroU8> {
		let (remaining, number) = digit1(input)?;
		let number: NonZeroU8 = number.parse().map_err(|_err| {
			nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
		})?;
		Ok((remaining, number))
	}

	fn parse_generation(input: &str) -> IResult<&str, Generation> {
		let (remaining, number) = delimited(tag("("), non_zero_u8, ordinal)(input)?;
		let (remaining, _) = tag(" generation)")(remaining)?; // consume the closing parenthesis

		Ok((remaining, Generation(number)))
	}

	fn parse_iphone_variant(input: &str) -> IResult<&str, IPhoneVariant> {
		match parse_iphone_discriminate(input)? {
			(remaining, true) => {
				// parse SE (3rd generation)
				// and parse <number>
				let (remaining, se_or_digit) =
					delimited(space1, alt((tag("SE"), digit1)), space0)(remaining)?;

				match se_or_digit {
					"SE" => {
						let (remaining, generation) = delimited(space0, parse_generation, space0)(remaining)?;

						Ok((remaining, IPhoneVariant::SE { generation }))
					}
					digit => {
						let digit = digit.parse().map_err(|_err| {
							nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
						})?;
						Ok((remaining, IPhoneVariant::Num(digit)))
					}
				}
			}
			(_, false) => {
				// MARK: Handle non-iphone device parsing here
				Err(nom::Err::Error(nom::error::Error::new(
					input,
					nom::error::ErrorKind::Tag,
				)))
			}
		}
	}

	fn parse_device_name(input: &str) -> IResult<&str, DeviceName> {
		let (remaining, device) = parse_iphone_variant(input)?;
		let plus = tag::<&str, &str, nom::error::Error<_>>("Plus")(remaining).is_ok();
		let pro = tag::<&str, &str, nom::error::Error<_>>("Pro")(remaining).is_ok();
		let max = tag::<&str, &str, nom::error::Error<_>>("Max")(remaining).is_ok();
		Ok((
			remaining,
			DeviceName::IPhone {
				variant: device,
				plus,
				pro,
				max,
			},
		))
	}

	impl FromStr for DeviceName {
		type Err = DeviceNameParseError;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			let (remaining, device) = parse_device_name(s).map_err(|e| {
				debug!("Failed to parse device name: {:?}", e);
				DeviceNameParseError::ParsingFailed
			})?;
			if !remaining.is_empty() {
				return Err(DeviceNameParseError::ParsingFailed);
			}

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
		fn parse_device_name() {
			let examples = include!("../../tests/names.json");
			for example in examples.iter() {
				let output = example.parse::<DeviceName>();
				match output {
					Ok(d) => debug!("Parsed device: {:?} from {}", d, example),
					Err(e) => panic!("Failed to parse {:?}: {}", example, e),
				}
			}
		}
	}
}
