use crate::prelude::*;

/// E.g. M1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MGen(NonZeroU8);

impl MGen {
	pub fn get_u8(&self) -> u8 {
		self.0.get()
	}

	pub fn get(&self) -> NonZeroU8 {
		self.0
	}

	fn new(num: NonZeroU8) -> Self {
		Self(num)
	}

	#[cfg(test)]
	pub(super) fn new_testing(num: NonZeroU8) -> Self {
		Self::new(num)
	}
}

/// parses (M1) -> MStatus(NonZeroU8(1))
fn parse_m_status(input: &str) -> IResult<&str, MGen> {
	delimited(tag("(M"), map(NonZeroU8::nom_from_str, MGen::new), tag(")"))(input)
}

impl Display for MGen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "(M{})", self.get())
	}
}

impl NomFromStr for MGen {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		parse_m_status(input)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_hard_coded() {
		let examples = ["(M1)", "(M2)"];
		for example in examples {
			let (remaining, parsed) = parse_m_status(example).unwrap();
			assert!(remaining.is_empty());
			assert_eq!(parsed.to_string(), example);
		}
	}
}
