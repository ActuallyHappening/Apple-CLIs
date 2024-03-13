use crate::shared::prelude::*;

/// Ordered from oldest to newest.
/// newest > oldest
#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants, PartialOrd, Ord)]
pub enum IPhoneVariant {
	SE {
		generation: Generation,
	},

	Number {
		num: NonZeroU8,
		plus: bool,
		pro: bool,
		max: bool,
	},
}

impl NomFromStr for IPhoneVariant {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		let (remaining, discriminate) = alt((
			value(IPhoneVariantDiscriminants::SE, ws(tag("SE"))),
			value(IPhoneVariantDiscriminants::Number, peek(ws(digit1))),
		))(input)?;

		match discriminate {
			IPhoneVariantDiscriminants::SE => {
				let (remaining, generation) = Generation::nom_from_str(remaining)?;
				Ok((remaining, IPhoneVariant::SE { generation }))
			}
			IPhoneVariantDiscriminants::Number => {
				let (remaining, num) = NonZeroU8::nom_from_str(remaining)?;
				let (remaining, plus) = alt((value(true, ws(tag("Plus"))), success(false)))(remaining)?;
				let (remaining, pro) = alt((value(true, ws(tag("Pro"))), success(false)))(remaining)?;
				let (remaining, max) = alt((value(true, ws(tag("Max"))), success(false)))(remaining)?;
				Ok((
					remaining,
					IPhoneVariant::Number {
						num,
						plus,
						pro,
						max,
					},
				))
			}
		}
	}
}

impl Display for IPhoneVariant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			IPhoneVariant::SE { generation } => write!(f, "SE {}", generation),
			IPhoneVariant::Number {
				num,
				plus,
				pro,
				max,
			} => {
				write!(f, "{}", num)?;
				if *plus {
					write!(f, " Plus")?;
				}
				if *pro {
					write!(f, " Pro")?;
				}
				if *max {
					write!(f, " Max")?;
				}
				Ok(())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn iphone_ordering() {
		let old = IPhoneVariant::SE {
			generation: Generation::new(1),
		};
		let new = IPhoneVariant::Number {
			num: NonZeroU8::new(69).unwrap(),
			plus: true,
			pro: true,
			max: true,
		};
		assert!(new > old);
	}
}
