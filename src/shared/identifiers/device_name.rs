use crate::{nom_from_str, prelude::*};
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
			map(ws(IPadVariant::nom_from_str), DeviceName::IPad),
			map(ws(IPhoneVariant::nom_from_str), DeviceName::IPhone),
			map(rest, |s: &str| DeviceName::UnImplemented(s.to_owned())),
		))(input)
	}
}

nom_from_str!(DeviceName);

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
			DeviceName::IPhone(variant) => write!(f, "{}", variant),
			DeviceName::IPad(variant) => write!(f, "{}", variant),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::shared::assert_nom_parses;

	use super::*;

	#[test]
	fn test_parse_device_name() {
		let examples = include!("../../../tests/names.json");
		assert_nom_parses::<DeviceName>(examples, |d| d.parsed_successfully())
	}
}
