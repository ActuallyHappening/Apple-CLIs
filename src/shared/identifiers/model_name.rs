use crate::prelude::*;

/// Isomorphic to [DeviceName].
/// [Deserialize]s and [Serialize]s from a [String] representation.
#[derive(
	Debug, Clone, PartialEq, derive_more::Display, derive_more::From, Serialize, Deserialize,
)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub enum ModelName {
	IPhone(IPhoneVariant),

	IPad(IPadVariant),

	#[from(ignore)]
	#[doc = include_doc!(todo)]
	UnImplemented(String),
}

impl From<ModelName> for String {
	#[tracing::instrument(level = "trace", skip(variant))]
	fn from(variant: ModelName) -> Self {
		variant.to_string()
	}
}

impl TryFrom<&str> for ModelName {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: &str) -> std::prelude::v1::Result<Self, Self::Error> {
		value.parse()
	}
}

impl TryFrom<String> for ModelName {
	type Error = <Self as FromStr>::Err;

	fn try_from(value: String) -> std::prelude::v1::Result<Self, Self::Error> {
		value.parse()
	}
}

impl_from_str_nom!(ModelName);

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
