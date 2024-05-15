use std::hash::Hash;

use crate::prelude::*;

/// Only considers the generation number for [Hash] and [PartialEq].
#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct NumGeneration {
	num: NonZeroU8,
	short: bool,
}

impl PartialEq for NumGeneration {
	fn eq(&self, other: &Self) -> bool {
		self.num == other.num
	}
}

impl Hash for NumGeneration {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.num.hash(state);
	}
}

impl NumGeneration {
	const fn ordinal(&self) -> &str {
		match self.num.get() {
			1 => "st",
			2 => "nd",
			3 => "rd",
			_ => "th",
		}
	}

	#[cfg_attr(not(test), allow(dead_code))]
	pub(crate) fn long(number: impl Into<u8>) -> Self {
		Self {
			num: NonZeroU8::new(number.into()).unwrap(),
			short: false,
		}
	}

	fn short(num: impl Into<u8>) -> Self {
		Self {
			num: NonZeroU8::new(num.into()).unwrap(),
			short: true,
		}
	}

	pub fn get(&self) -> u8 {
		self.num.get()
	}
}

#[tracing::instrument(level = "trace", skip(input))]
fn ordinal(input: &str) -> IResult<&str, &str> {
	alt((tag("st"), tag("nd"), tag("rd"), tag("th")))(input)
}

fn generation_brackets(input: &str) -> IResult<&str, NumGeneration> {
	delimited(
		ws(tag("(")),
		map(NonZeroU8::nom_from_str, NumGeneration::long),
		preceded(ws(ordinal), tag("generation)")),
	)(input)
}

fn generation_model(input: &str) -> IResult<&str, NumGeneration> {
	terminated(
		map(NonZeroU8::nom_from_str, NumGeneration::short),
		tag("G"),
	)(input)
}

impl NomFromStr for NumGeneration {
	#[tracing::instrument(level = "trace", skip(input))]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((generation_brackets, generation_model))(input)
	}
}

impl Display for NumGeneration {
	#[tracing::instrument(level = "trace", skip(self, f))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.short {
			false => write!(f, "({}{} generation)", self.get(), self.ordinal()),
			true => write!(f, "{}G", self.get()),
		}
	}
}

#[cfg(test)]
mod tests {
	use tracing::debug;

	use super::*;

	#[test]
	fn generation_ordering() {
		let old = NumGeneration::long(NonZeroU8::new(1).unwrap());
		let newer = NumGeneration::short(NonZeroU8::new(2).unwrap());
		assert!(newer > old);

		let old = NumGeneration::short(NonZeroU8::new(1).unwrap());
		let newer = NumGeneration::long(NonZeroU8::new(2).unwrap());
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
			"3G",
			"69G",
		];
		for example in examples.iter() {
			let output = NumGeneration::nom_from_str(example);
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
