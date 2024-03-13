use crate::shared::prelude::*;
use super::NomFromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenSize {
	inches: f32,
}

impl ScreenSize {
	fn new(inches: f32) -> Self {
		Self { inches }
	}
}

impl NomFromStr for ScreenSize {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, inches) = delimited(tag("("), float, tag("-inch)"))(input)?;
		Ok((remaining, ScreenSize::new(inches)))
	}
}
