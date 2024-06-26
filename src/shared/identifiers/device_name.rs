use crate::prelude::*;

/// [Deserialize]s from a [String] representation.
#[derive(
	Debug, Clone, PartialEq, derive_more::Display, derive_more::From, Serialize, Deserialize,
)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub enum DeviceName {
	IPhone(IPhoneVariant),

	IPad(IPadVariant),

	#[from(ignore)]
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	UnImplemented(String),
}

impl From<DeviceName> for String {
	#[tracing::instrument(level = "trace", skip(variant))]
	fn from(variant: DeviceName) -> Self {
		variant.to_string()
	}
}

impl TryFrom<&str> for DeviceName {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
		value.parse()
	}
}

impl TryFrom<String> for DeviceName {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
		value.parse()
	}
}

impl_from_str_nom!(DeviceName);

impl From<&IPhoneVariant> for DeviceName {
	#[tracing::instrument(level = "trace", skip(variant))]
	fn from(variant: &IPhoneVariant) -> Self {
		Self::IPhone(*variant)
	}
}

impl From<&IPadVariant> for DeviceName {
	#[tracing::instrument(level = "trace", skip(variant))]
	fn from(variant: &IPadVariant) -> Self {
		Self::IPad(*variant)
	}
}

impl From<&DeviceName> for DeviceName {
	#[tracing::instrument(level = "trace", skip(variant))]
	fn from(variant: &DeviceName) -> Self {
		variant.clone()
	}
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

	pub fn get_ipad(&self) -> Option<&IPadVariant> {
		match self {
			DeviceName::IPad(ipad) => Some(ipad),
			_ => None,
		}
	}

	pub fn get_iphone(&self) -> Option<&IPhoneVariant> {
		match self {
			DeviceName::IPhone(iphone) => Some(iphone),
			_ => None,
		}
	}
}

impl NomFromStr for DeviceName {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			map(ws(IPadVariant::nom_from_str), DeviceName::IPad),
			map(ws(IPhoneVariant::nom_from_str), DeviceName::IPhone),
			map(rest, |s: &str| DeviceName::UnImplemented(s.to_owned())),
		))(input)
	}
}

#[cfg(test)]
mod tests {
	use crate::shared::assert_nom_parses;

	use super::*;

	#[test]
	fn hardcoded_parse_device_names() {
		let examples = [
			"iPad Air 11-inch (M2)",
			"iPad Pro (11-inch) (4th generation)",
		];
		assert_nom_parses::<DeviceName>(examples, |d| d.parsed_successfully());
	}

	#[test]
	fn generated_parse_device_name() {
		let examples = include!("../../../tests/device-names.json");
		assert_nom_parses::<DeviceName>(examples, |d| d.parsed_successfully())
	}
}
