use std::fmt::Display;

use crate::shared::prelude::*;
use super::NomFromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

impl Display for ScreenSize {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}-inch)", self.inches)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn screen_size_ordering() {
		let small = ScreenSize::new(5.0);
		let large = ScreenSize::new(6.0);
		assert!(large > small);
	}
}