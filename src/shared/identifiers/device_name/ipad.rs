use crate::prelude::*;

/// Loosely ordered from oldest to newest.
/// newest > oldest
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumDiscriminants, PartialOrd, Ord)]
pub enum IPadVariant {
	Air {
		generation: Generation,
	},
	Mini {
		generation: Generation,
	},
	Plain {
		generation: Generation,
	},
	Pro {
		size: ScreenSize,
		generation: Generation,
	},
}

impl NomFromStr for IPadVariant {
	#[tracing::instrument(level = "trace", skip(input))]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, discriminate) = preceded(
			tag("iPad"),
			alt((
				value(IPadVariantDiscriminants::Mini, ws(tag("mini"))),
				value(IPadVariantDiscriminants::Air, ws(tag("Air"))),
				value(IPadVariantDiscriminants::Pro, ws(tag("Pro"))),
				success(IPadVariantDiscriminants::Plain),
			)),
		)(input)?;

		#[cfg(test)]
		trace!(
			?discriminate,
			"Discriminant found for parsing [IPadVariant]"
		);

		match discriminate {
			IPadVariantDiscriminants::Air => {
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				Ok((remaining, IPadVariant::Air { generation }))
			}
			IPadVariantDiscriminants::Mini => {
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				Ok((remaining, IPadVariant::Mini { generation }))
			}
			IPadVariantDiscriminants::Plain => {
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				Ok((remaining, IPadVariant::Plain { generation }))
			}
			IPadVariantDiscriminants::Pro => {
				let (remaining, size) = ws(ScreenSize::nom_from_str)(remaining)?;
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				Ok((remaining, IPadVariant::Pro { size, generation }))
			}
		}
	}
}

impl Display for IPadVariant {
	#[tracing::instrument(level = "trace", skip(self, f))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			IPadVariant::Pro { size, generation } => write!(f, "iPad Pro {} {}", size, generation),
			IPadVariant::Mini { generation } => write!(f, "iPad mini {}", generation),
			IPadVariant::Air { generation } => write!(f, "iPad Air {}", generation),
			IPadVariant::Plain { generation } => write!(f, "iPad {}", generation),
		}
	}
}

#[cfg(test)]
mod test {
	use crate::shared::assert_nom_parses;

	use super::*;

	#[test]
	fn ipad_ordering() {
		let old = IPadVariant::Plain {
			generation: Generation::long(NonZeroU8::new(1).unwrap()),
		};
		let new = IPadVariant::Pro {
			size: ScreenSize::long(12.9),
			generation: Generation::long(NonZeroU8::new(2).unwrap()),
		};
		assert!(new > old);
	}

	#[test]
	fn partial_parsing() {
		let examples = [
			"iPad Air (5th generation)",
			"iPad (10th generation)",
			"iPad mini (6th generation)",
			"iPad Pro (11-inch) (4th generation)",
			"iPad Pro (12.9-inch) (6th generation)",
		];
		assert_nom_parses::<IPadVariant>(examples, |_| true)
	}
}
