use crate::prelude::*;

/// [Deserialize]s from a [String] representation.
#[derive(Debug, Clone, PartialEq, derive_more::Display, derive_more::From)]
pub enum DeviceName {
	IPhone(IPhoneVariant),

	IPad(IPadVariant),

	#[from(ignore)]
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	UnImplemented(String),
}

nom_from_str!(DeviceName);
impl_str_serde!(DeviceName);

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
	#[tracing::instrument(level = "trace", skip(input))]
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
	fn test_parse_device_name() {
		let examples = include!("../../../tests/device-names.json");
		assert_nom_parses::<DeviceName>(examples, |d| d.parsed_successfully())
	}
}
