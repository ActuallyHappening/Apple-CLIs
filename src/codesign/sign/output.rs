use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub enum SignOutput {
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	UnImplemented(String),
}

impl_from_str_nom!(SignOutput);

impl NomFromStr for SignOutput {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		map(rest, |s: &str| SignOutput::UnImplemented(s.to_owned()))(input)
	}
}
