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
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceName {
	IPhone(IPhoneVariant),

	IPad(IPadVariant),

	#[doc = include_str!("../../../docs/inline/TODO.md")]
	UnImplemented(String),
}

pub use iphone::*;
mod iphone;

pub use ipad::*;
mod ipad;

impl DeviceName {
	pub fn parsed_successfully(&self) -> bool {
		!matches!(self, DeviceName::UnImplemented(_))
	}

	pub fn is_iphone(&self) -> bool {
		matches!(self, DeviceName::IPhone(_))
	}

	pub fn is_ipad(&self) -> bool {
		matches!(self, DeviceName::IPad(_))
	}
}

impl From<IPhoneVariant> for DeviceName {
	fn from(variant: IPhoneVariant) -> Self {
		DeviceName::IPhone(variant)
	}
}

impl From<IPadVariant> for DeviceName {
	fn from(variant: IPadVariant) -> Self {
		DeviceName::IPad(variant)
	}
}

impl NomFromStr for DeviceName {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			map(
				preceded(ws(tag("iPad")), IPadVariant::nom_from_str),
				DeviceName::IPad,
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
			DeviceName::IPad(variant) => write!(f, "iPad {}", variant),
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
						&device.to_string(),
						example,
						"The parsed device name {:?} from {:?} displayed {}",
						device,
						example,
						&device.to_string()
					);
				}
				Err(e) => panic!("Failed to parse {:?}: {}", example, e),
			}
		}
	}
}
