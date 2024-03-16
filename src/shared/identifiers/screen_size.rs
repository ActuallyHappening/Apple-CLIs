use std::fmt::Display;

use nom::combinator::all_consuming;

use super::NomFromStr;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScreenSize {
	/// divide by 10 to get number of actual inches
	ten_inches: u16,
	short: bool,
}

impl ScreenSize {
	#[tracing::instrument(level = "trace", skip(inches))]
	pub(crate) fn long(inches: f32) -> Self {
		let inches = inches * 10.0;

		Self {
			ten_inches: inches as u16,
			short: false,
		}
	}

	#[tracing::instrument(level = "trace", skip(inches))]
	pub(crate) fn short(inches: f32) -> Self {
		let inches = inches * 10.0;

		Self {
			ten_inches: inches as u16,
			short: true,
		}
	}

	#[tracing::instrument(level = "trace", skip(self))]
	pub fn inches(&self) -> f32 {
		self.ten_inches as f32 / 10.0
	}
}

fn screen_size_long(input: &str) -> IResult<&str, ScreenSize> {
	all_consuming(delimited(
		ws(tag("(")),
		map(float, ScreenSize::long),
		ws(tag("-inch)")),
	))(input)
}

fn screen_size_short(input: &str) -> IResult<&str, ScreenSize> {
	all_consuming(delimited(
		ws(tag("(")),
		map(float, ScreenSize::short),
		ws(tag("\")")),
	))(input)
}

impl NomFromStr for ScreenSize {
	#[tracing::instrument(level = "trace", skip(input))]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			screen_size_long,
			screen_size_short
		))(input)
	}
}

impl Display for ScreenSize {
	#[tracing::instrument(level = "trace", skip(self, f))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.short {
			false => write!(f, "({}-inch)", self.inches()),
			true => write!(f, "({}\")", self.inches()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn screen_size_parses() {
		let examples = ["(5.5-inch)", "(6.1\")"];
		for example in examples {
			let (remaining, parsed) = ScreenSize::nom_from_str(example).unwrap();
			assert!(remaining.is_empty());
			assert_eq!(parsed.to_string(), example);
		}
	}

	#[test]
	fn screen_size_ordering() {
		let small = ScreenSize::long(5.0);
		let large = ScreenSize::short(6.0);
		assert!(large > small);

		let small = ScreenSize::long(5.0);
		let large = ScreenSize::short(6.0);
		assert!(large > small);
	}
}
