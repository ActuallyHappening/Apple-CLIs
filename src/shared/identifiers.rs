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
		combinator::{success, value},
		sequence::{delimited, preceded, terminated},
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
		alt((value(true, tag("iPhone")), success(false)))(input)
	}

	#[derive(Debug)]
	pub enum IPhoneVariant {
		SE {
			generation: Generation,
		},
		Num(NonZeroU8),
		#[doc = include_str!("../../docs/TODO.md")]
		UnImplemented,
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
			(_remaining, false) => {
				// MARK: Handle non-iphone device parsing here
				Ok(("", IPhoneVariant::UnImplemented))
			}
		}
	}

	fn parse_device_name(input: &str) -> IResult<&str, DeviceName> {
		let (remaining, device) = parse_iphone_variant(input)?;
		if let IPhoneVariant::UnImplemented = device {
			return Ok(("", DeviceName::UnImplemented(input.to_string())));
		}

		let (remaining, _) = space0(remaining)?;
		let (remaining, plus) = alt((value(true, tag("Plus")), success(false)))(remaining)?;
		let (remaining, pro) =
			alt((value(true, terminated(tag("Pro"), space0)), success(false)))(remaining)?;
		let (remaining, max) =
			alt((value(true, terminated(tag("Max"), space0)), success(false)))(remaining)?;
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
