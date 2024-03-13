use crate::shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Generation(NonZeroU8);

fn ordinal(input: &str) -> IResult<&str, &str> {
	alt((tag("st"), tag("nd"), tag("rd"), tag("th")))(input)
}

impl NomFromStr for Generation {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, number) = delimited(tag("("), NonZeroU8::nom_from_str, ordinal)(input)?;
		let (remaining, _) = ws(tag("generation)"))(remaining)?; // consume the closing parenthesis

		Ok((remaining, Generation(number)))
	}
}

#[cfg(test)]
mod tests {
	use tracing::debug;

use super::*;

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
				}
				Err(e) => panic!("Failed to parse {:?}: {}", example, e),
			}
		}
	}
}
