use crate::{impl_str_serde, nom_from_str, prelude::*};

/// Isomorphic to [DeviceName].
/// [Deserialize]s and [Serialize]s from a [String] representation.
#[derive(Debug, Clone, PartialEq, derive_more::Display, derive_more::From)]
pub enum ModelName {
	IPhone(IPhoneVariant),

	IPad(IPadVariant),

	#[from(ignore)]
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	UnImplemented(String),
}

nom_from_str!(ModelName);
impl_str_serde!(ModelName);

impl ModelName {
	pub fn is_iphone(&self) -> bool {
		matches!(self, ModelName::IPhone(_))
	}
	pub fn is_ipad(&self) -> bool {
		matches!(self, ModelName::IPad(_))
	}

	pub fn parsed_successfully(&self) -> bool {
		!matches!(self, ModelName::UnImplemented(_))
	}
}

impl NomFromStr for ModelName {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			map(IPadVariant::nom_from_str, ModelName::from),
			map(IPhoneVariant::nom_from_str, ModelName::from),
			map(rest, |s: &str| ModelName::UnImplemented(s.to_owned())),
		))(input)
	}
}

#[cfg(test)]
mod tests {
	use crate::shared::assert_nom_parses;

	use super::ModelName;

	#[test]
	fn model_names_parse() {
		let examples = include!(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/tests/model-names.json"
		));
		assert_nom_parses::<ModelName>(examples, |input| input.parsed_successfully());
	}
}
