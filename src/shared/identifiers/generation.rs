use std::fmt::Display;

use crate::shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Generation(NonZeroU8);

#[tracing::instrument(level = "trace", skip(input))]
fn ordinal(input: &str) -> IResult<&str, &str> {
	alt((tag("st"), tag("nd"), tag("rd"), tag("th")))(input)
}

impl Generation {
	const fn ordinal(&self) -> &str {
		match self.0.get() {
			1 => "st",
			2 => "nd",
			3 => "rd",
			_ => "th",
		}
	}

	#[tracing::instrument(level = "trace", skip(number))]
	#[cfg_attr(not(test), allow(dead_code))]
	pub(crate) fn new(number: impl Into<u8>) -> Self {
		Self(NonZeroU8::new(number.into()).unwrap())
	}

	pub fn get(&self) -> u8 {
		self.0.get()
	}
}

impl NomFromStr for Generation {
	#[tracing::instrument(level = "trace", skip(input))]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, number) = delimited(tag("("), NonZeroU8::nom_from_str, ordinal)(input)?;
		let (remaining, _) = ws(tag("generation)"))(remaining)?; // consume the closing parenthesis

		Ok((remaining, Generation(number)))
	}
}

impl Display for Generation {
	#[tracing::instrument(level = "trace", skip(self, f))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({}{} generation)", self.0, self.ordinal())
	}
}

#[cfg(test)]
mod tests {
	use tracing::debug;

	use super::*;

	#[test]
	fn generation_ordering() {
		let old = Generation(NonZeroU8::new(1).unwrap());
		let newer = Generation(NonZeroU8::new(2).unwrap());
		assert!(newer > old);
	}

	#[test]
	fn test_parse_ordinal() {
		let examples = ["st", "nd", "th"];
		for example in examples.iter() {
			let output = ordinal(example);
			match output {
				Ok((remaining, _)) => {
					debug!("Parsed ordinal from {}: {:?}", example, remaining)
				}
				Err(e) => panic!("Failed to parse {:?}: {}", example, e),
			}
		}
	}

	#[test]
	fn test_parse_generation() {
		let examples = [
			"(1st generation)",
			"(2nd generation)",
			"(3rd generation)",
			"(4th generation)",
		];
		for example in examples.iter() {
			let output = Generation::nom_from_str(example);
			match output {
				Ok((remaining, generation)) => {
					debug!(
						"Parsed generation: {:?} from {} [remaining: {}]",
						generation, example, remaining
					);
					assert!(
						remaining.is_empty(),
						"Remaining was not empty: {}",
						remaining
					);
					assert_eq!(&format!("{}", generation), example);
				}
				Err(e) => panic!("Failed to parse {:?}: {}", example, e),
			}
		}
	}
}
