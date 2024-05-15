use crate::prelude::*;

/// [Generation::M] is considered newer / greater than [Generation::Num].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Generation {
	Num(NumGeneration),
	M(MGen),
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn generation_ordering() {
		let lower = Generation::Num(NumGeneration::long(1));
		let middle = Generation::M(MGen::new_testing(NonZeroU8::new(1).unwrap()));
		let higher = Generation::M(MGen::new_testing(NonZeroU8::new(3).unwrap()));

		assert!(lower < middle);
		assert!(middle < higher);
		assert!(lower < higher);
	}
}