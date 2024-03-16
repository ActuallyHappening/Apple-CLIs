use std::fmt::Display;

use super::NomFromStr;
use crate::shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScreenSize {
	/// divide by 10 to get number of actual inches
	ten_inches: u16,
}

impl ScreenSize {
	#[tracing::instrument(level = "trace", skip(inches))]
	pub(crate) fn new(inches: f32) -> Self {
		let inches = inches * 10.0;

		Self {
			ten_inches: inches as u16,
		}
	}

	#[tracing::instrument(level = "trace", skip(self))]
	pub fn inches(&self) -> f32 {
		self.ten_inches as f32 / 10.0
	}
}

impl NomFromStr for ScreenSize {
	#[tracing::instrument(level = "trace", skip(input))]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, inches) = delimited(tag("("), float, tag("-inch)"))(input)?;
		Ok((remaining, ScreenSize::new(inches)))
	}
}

impl Display for ScreenSize {
	#[tracing::instrument(level = "trace", skip(self, f))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}-inch)", self.inches())
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
