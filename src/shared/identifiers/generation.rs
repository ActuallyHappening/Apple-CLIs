use crate::prelude::*;

// pub use m_gen::MGen;
use m_gen::MGen;
// pub use num_generation::NumGeneration;
use num_generation::NumGeneration;

pub mod m_gen;
pub mod num_generation;

/// [Generation::M] is considered newer / greater than [Generation::Num].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Generation {
	Num(NumGeneration),
	M(MGen),
}

impl Generation {
	#[cfg(test)]
	pub(super) fn testing_num(num: NonZeroU8) -> Self {
		Generation::Num(NumGeneration::testing_new(num))
	}
}

impl From<NumGeneration> for Generation {
	fn from(value: NumGeneration) -> Self {
		Self::Num(value)
	}
}

impl From<MGen> for Generation {
	fn from(value: MGen) -> Self {
		Self::M(value)
	}
}

impl NomFromStr for Generation {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			map(NumGeneration::nom_from_str, Generation::Num),
			map(MGen::nom_from_str, Generation::M),
		))(input)
	}
}

impl Display for Generation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Generation::Num(num) => write!(f, "{}", num),
			Generation::M(m) => write!(f, "{}", m),
		}
	}
}

#[cfg(test)]
mod test {
	use self::shared::assert_nom_parses;

	use super::*;

	#[test]
	fn generation_ordering() {
		let lower = Generation::Num(NumGeneration::testing_new(1.try_into().unwrap()));
		let middle = Generation::M(MGen::new_testing(NonZeroU8::new(1).unwrap()));
		let higher = Generation::M(MGen::new_testing(NonZeroU8::new(3).unwrap()));

		assert!(lower < middle);
		assert!(middle < higher);
		assert!(lower < higher);
	}

	#[test]
	fn test_generation_hard_coded() {
		let examples = ["(6th generation)", "(M2)"];

		assert_nom_parses::<Generation>(examples, |_| true)
	}
}
