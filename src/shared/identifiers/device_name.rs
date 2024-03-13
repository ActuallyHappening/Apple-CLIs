use super::{generation::Generation, prelude::*, screen_size::ScreenSize};
use std::num::NonZeroU8;

use strum::EnumDiscriminants;

use super::{ws, NomFromStr};

#[derive(thiserror::Error, Debug)]
pub enum DeviceNameParseError {
	#[error("Failed to parse device name")]
	ParsingFailed(#[source] nom::error::Error<String>),
}

#[derive(Debug)]
pub enum DeviceName {
	IPhone(IPhoneVariant),

	Ipad(IPadVariant),

	#[doc = include_str!("../../../docs/TODO.md")]
	UnImplemented(String),
}

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
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

	#[doc = include_str!("../../../docs/TODO.md")]
	UnImplemented {
		input: String,
	},
}

#[derive(Debug, Clone, PartialEq, EnumDiscriminants)]
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

impl DeviceName {
	pub fn parsed_successfully(&self) -> bool {
		!matches!(self, DeviceName::UnImplemented(_))
	}
}

impl NomFromStr for IPadVariant {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, discriminate) = alt((
			value(IPadVariantDiscriminants::Mini, ws(tag("mini"))),
			value(IPadVariantDiscriminants::Air, ws(tag("Air"))),
			value(IPadVariantDiscriminants::Pro, ws(tag("Pro"))),
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
				let (remaining, size) = ws(ScreenSize::nom_from_str)(remaining)?;
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				Ok((remaining, IPadVariant::Pro { size, generation }))
			}
		}
	}
}

impl NomFromStr for IPhoneVariant {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		eprintln!("Parsing IPhoneVariant from {:?}", input);
		let (remaining, discriminate) = alt((
			value(IPhoneVariantDiscriminants::SE, ws(tag("SE"))),
			value(IPhoneVariantDiscriminants::Number, peek(ws(digit1))),
			success(IPhoneVariantDiscriminants::UnImplemented),
		))(input)?;

		match discriminate {
			IPhoneVariantDiscriminants::SE => {
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				Ok((remaining, IPhoneVariant::SE { generation }))
			}
			IPhoneVariantDiscriminants::Number => {
				let (remaining, num) = NonZeroU8::nom_from_str(remaining)?;
				let (remaining, plus) = alt((value(false, ws(tag("Plus"))), success(true)))(remaining)?;
				let (remaining, pro) = alt((value(false, ws(tag("Pro"))), success(true)))(remaining)?;
				let (remaining, max) = alt((value(false, ws(tag("Max"))), success(true)))(remaining)?;
				Ok((
					remaining,
					IPhoneVariant::Number {
						num,
						plus,
						pro,
						max,
					},
				))
			}
			IPhoneVariantDiscriminants::UnImplemented => Ok((
				remaining,
				IPhoneVariant::UnImplemented {
					input: remaining.to_owned(),
				},
			)),
		}
	}
}

impl NomFromStr for DeviceName {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			map(
				preceded(ws(tag("iPad")), IPadVariant::nom_from_str),
				DeviceName::Ipad,
			),
			map(
				preceded(ws(tag("iPhone")), IPhoneVariant::nom_from_str),
				DeviceName::IPhone,
			),
			map(rest, |s: &str| DeviceName::UnImplemented(s.to_owned())),
		))(input)
	}
}

#[cfg(test)]
mod tests {
	use tracing::debug;

	use super::*;

	#[test]
	fn test_parse_device_name() {
		let examples = include!("../../../tests/names.json");
		for example in examples.iter() {
			let output = DeviceName::nom_from_str(example);
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
					assert!(
						device.parsed_successfully(),
						"{:?} was not parsed successfully",
						device
					);
				}
				Err(e) => panic!("Failed to parse {:?}: {}", example, e),
			}
		}
	}
}