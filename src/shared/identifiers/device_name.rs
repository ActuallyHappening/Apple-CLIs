use super::{generation::Generation, screen_size::ScreenSize};
use crate::shared::prelude::*;
use std::{fmt::Display, num::NonZeroU8, str::FromStr};

use serde::{Deserialize, Deserializer};
use strum::EnumDiscriminants;

#[derive(thiserror::Error, Debug)]
pub enum DeviceNameParseError {
	#[error("Failed to parse device name")]
	ParsingFailed(#[source] nom::Err<nom::error::Error<String>>),

	#[error(
		"The parsed string was not completely consumed, with {:?} left from {:?}. Parsed: {:?}",
		input,
		remaining,
		parsed
	)]
	RemainingNotEmpty {
		input: String,
		remaining: String,
		parsed: DeviceName,
	},
}

/// [Deserialize]s from a [String] representation.
#[derive(Debug)]
pub enum DeviceName {
	IPhone(IPhoneVariant),

	Ipad(IPadVariant),

	#[doc = include_str!("../../../docs/TODO.md")]
	UnImplemented(String),
}

use iphone::*;
mod iphone {
	use super::*;

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

	impl NomFromStr for IPhoneVariant {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
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
					let (remaining, plus) = alt((value(true, ws(tag("Plus"))), success(false)))(remaining)?;
					let (remaining, pro) = alt((value(true, ws(tag("Pro"))), success(false)))(remaining)?;
					let (remaining, max) = alt((value(true, ws(tag("Max"))), success(false)))(remaining)?;
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

	impl Display for IPhoneVariant {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				IPhoneVariant::UnImplemented { input } => write!(f, "{}", input),
				IPhoneVariant::SE { generation } => write!(f, "SE {}", generation),
				IPhoneVariant::Number {
					num,
					plus,
					pro,
					max,
				} => {
					write!(f, "{}", num)?;
					if *plus {
						write!(f, " Plus")?;
					}
					if *pro {
						write!(f, " Pro")?;
					}
					if *max {
						write!(f, " Max")?;
					}
					Ok(())
				}
			}
		}
	}
}

use ipad::*;
mod ipad {
	use super::*;

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

	impl Display for IPadVariant {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				IPadVariant::Mini { generation } => write!(f, "mini {}", generation),
				IPadVariant::Air { generation } => write!(f, "Air {}", generation),
				IPadVariant::Plain { generation } => write!(f, "{}", generation),
				IPadVariant::Pro { size, generation } => write!(f, "Pro {} {}", size, generation),
			}
		}
	}
}

impl DeviceName {
	pub fn parsed_successfully(&self) -> bool {
		!matches!(self, DeviceName::UnImplemented(_))
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

impl FromStr for DeviceName {
	type Err = DeviceNameParseError;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match DeviceName::nom_from_str(input) {
			Ok((remaining, device)) => {
				if remaining.is_empty() {
					Ok(device)
				} else {
					Err(DeviceNameParseError::RemainingNotEmpty {
						input: input.to_owned(),
						remaining: remaining.to_owned(),
						parsed: device,
					})
				}
			}
			Err(e) => Err(DeviceNameParseError::ParsingFailed(e.to_owned())),
		}
	}
}

impl<'de> Deserialize<'de> for DeviceName {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let buf = String::deserialize(deserializer)?;

		// cron::Schedule::from_str(&buf).map_err(serde::de::Error::custom)
		DeviceName::from_str(&buf).map_err(serde::de::Error::custom)
	}
}

impl Display for DeviceName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DeviceName::UnImplemented(s) => write!(f, "{}", s),
			DeviceName::IPhone(variant) => write!(f, "iPhone {}", variant),
			DeviceName::Ipad(variant) => write!(f, "iPad {}", variant),
		}
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
					assert_eq!(
						&format!("{}", device),
						example,
						"The parsed device name {:?} from {:?} displayed {}",
						device,
						example,
						&format!("{}", device)
					);
				}
				Err(e) => panic!("Failed to parse {:?}: {}", example, e),
			}
		}
	}
}
