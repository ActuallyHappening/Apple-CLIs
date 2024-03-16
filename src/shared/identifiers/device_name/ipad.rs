use crate::shared::prelude::*;

/// Loosely ordered from oldest to newest.
/// newest > oldest
#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants, PartialOrd, Ord)]
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
		let (remaining, discriminate) = alt((
			value(IPadVariantDiscriminants::Mini, ws(tag("mini"))),
			value(IPadVariantDiscriminants::Air, ws(tag("Air"))),
			value(IPadVariantDiscriminants::Pro, ws(tag("Pro"))),
			success(IPadVariantDiscriminants::Plain),
		))(input)?;

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
			IPadVariant::Mini { generation } => write!(f, "mini {}", generation),
			IPadVariant::Air { generation } => write!(f, "Air {}", generation),
			IPadVariant::Plain { generation } => write!(f, "{}", generation),
			IPadVariant::Pro { size, generation } => write!(f, "Pro {} {}", size, generation),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ipad_ordering() {
		let old = IPadVariant::Plain {
			generation: Generation::new(NonZeroU8::new(1).unwrap()),
		};
		let new = IPadVariant::Pro {
			size: ScreenSize::new(12.9),
			generation: Generation::new(NonZeroU8::new(2).unwrap()),
		};
		assert!(new > old);
	}
}
