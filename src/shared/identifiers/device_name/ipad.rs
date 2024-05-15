use crate::prelude::*;

use self::identifiers::m_status::{MStatus, MaybeMStatus};

/// Loosely ordered from oldest to newest.
/// newest > oldest
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumDiscriminants, PartialOrd, Ord)]
pub enum IPadVariant {
	Air {
		generation: Generation,
		m_status: MaybeMStatus,
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
		/// For lossless parsing
		size_before_generation: bool,
		m_status: MaybeMStatus,
	},
}

impl NomFromStr for IPadVariant {
	#[tracing::instrument(level = "trace")]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, discriminate) = preceded(
			ws(tag("iPad")),
			cut(alt((
				value(IPadVariantDiscriminants::Mini, ws(tag("mini"))),
				value(IPadVariantDiscriminants::Air, ws(tag("Air"))),
				value(IPadVariantDiscriminants::Pro, ws(tag("Pro"))),
				success(IPadVariantDiscriminants::Plain),
			))),
		)(input)?;

		#[cfg(test)]
		trace!(
			?discriminate,
			remaining,
			input,
			"Discriminant found for parsing [IPadVariant]"
		);

		match discriminate {
			IPadVariantDiscriminants::Air => {
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				let (remaining, m_status) = ws(MaybeMStatus::nom_from_str)(remaining)?;
				Ok((
					remaining,
					IPadVariant::Air {
						generation,
						m_status,
					},
				))
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
				let (remaining, (size_before_generation, (size, generation))) = alt((
					map(
						pair(ws(ScreenSize::nom_from_str), ws(Generation::nom_from_str)),
						|v| (true, v),
					),
					map(
						pair(ws(Generation::nom_from_str), ws(ScreenSize::nom_from_str)),
						|(gen, ss)| (false, (ss, gen)),
					),
				))(remaining)?;
				let (remaining, m_status) = ws(MaybeMStatus::nom_from_str)(remaining)?;
				Ok((
					remaining,
					IPadVariant::Pro {
						size,
						generation,
						size_before_generation,
						m_status,
					},
				))
			}
		}
	}
}

impl Display for IPadVariant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			IPadVariant::Pro {
				size,
				generation,
				size_before_generation: true,
				m_status,
			} => write!(f, "iPad Pro {} {}{}", size, generation, m_status,),
			IPadVariant::Pro {
				size,
				generation,
				size_before_generation: false,
				m_status,
			} => write!(f, "iPad Pro {} {}{}", generation, size, m_status,),
			IPadVariant::Mini { generation } => write!(f, "iPad mini {}", generation),
			IPadVariant::Air {
				generation,
				m_status,
			} => write!(f, "iPad Air {}{}", generation, m_status),
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
			size: ScreenSize::long_brackets(12.9),
			generation: Generation::long(NonZeroU8::new(2).unwrap()),
			size_before_generation: false,
			m_status: MaybeMStatus::default_testing(),
		};
		assert!(new > old);
	}

	#[test]
	fn hard_coded_parsing() {
		let examples = [
			// "iPad Air (5th generation)",
			// "iPad (10th generation)",
			// "iPad mini (6th generation)",
			// "iPad Pro (11-inch) (4th generation)",
			// "iPad Pro (12.9-inch) (6th generation)",
			// "iPad Air 11-inch (M2)",
			// "iPad Air 13-inch (M2)",
			// "iPad Pro 11-inch (M4)",
			"iPad Pro 13-inch (M4)",
		];
		assert_nom_parses::<IPadVariant>(examples, |_| true)
	}
}
