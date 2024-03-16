use crate::shared::prelude::*;
use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

/// [Deserialize]s from a [String] representation.
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceName {
	IPhone(IPhoneVariant),

	IPad(IPadVariant),

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	UnImplemented(String),
}

pub use iphone::*;
mod iphone;

pub use ipad::*;
mod ipad;

impl DeviceName {
	#[tracing::instrument(level = "trace", skip(self))]
	pub fn parsed_successfully(&self) -> bool {
		!matches!(self, DeviceName::UnImplemented(_))
	}

	#[tracing::instrument(level = "trace", skip(self))]
	pub fn is_iphone(&self) -> bool {
		matches!(self, DeviceName::IPhone(_))
	}

	#[tracing::instrument(level = "trace", skip(self))]
	pub fn is_ipad(&self) -> bool {
		matches!(self, DeviceName::IPad(_))
	}
}

impl From<IPhoneVariant> for DeviceName {
	#[tracing::instrument(level = "trace", skip(variant))]
	fn from(variant: IPhoneVariant) -> Self {
		DeviceName::IPhone(variant)
	}
}

impl From<IPadVariant> for DeviceName {
	#[tracing::instrument(level = "trace", skip(variant))]
	fn from(variant: IPadVariant) -> Self {
		DeviceName::IPad(variant)
	}
}

impl NomFromStr for DeviceName {
	#[tracing::instrument(level = "trace", skip(input))]
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
	type Err = error::Error;

	#[tracing::instrument(level = "trace", skip(input))]
	fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
		match DeviceName::nom_from_str(input) {
			Ok((remaining, device)) => {
				if remaining.is_empty() {
					Ok(device)
				} else {
					Err(Error::ParsingRemainingNotEmpty {
						input: input.to_owned(),
						remaining: remaining.to_owned(),
						parsed_debug: format!("{:#?}", device),
					})
				}
			}
			Err(e) => Err(Error::ParsingFailed {
				err: e.to_owned(),
				name: "Device Name".into(),
			}),
		}
	}
}

impl<'de> Deserialize<'de> for DeviceName {
	#[tracing::instrument(level = "trace", skip(deserializer))]
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let buf = String::deserialize(deserializer)?;

		// cron::Schedule::from_str(&buf).map_err(serde::de::Error::custom)
		DeviceName::from_str(&buf).map_err(serde::de::Error::custom)
	}
}

// impl serialize
impl Serialize for DeviceName {
	#[tracing::instrument(level = "trace", skip(self, serializer))]
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl Display for DeviceName {
	#[tracing::instrument(level = "trace", skip(self, f))]
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
