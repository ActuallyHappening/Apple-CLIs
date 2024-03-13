use super::*;

	#[derive(Debug, Clone, PartialEq, EnumDiscriminants, PartialOrd)]
	pub enum IPadVariant {
		Pro {
			size: ScreenSize,
			generation: Generation,
		},
		Plain {
			generation: Generation,
		},
		Mini {
			generation: Generation,
		},
		Air {
			generation: Generation,
		},
	}

	impl NomFromStr for IPadVariant {
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
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			match self {
				IPadVariant::Mini { generation } => write!(f, "mini {}", generation),
				IPadVariant::Air { generation } => write!(f, "Air {}", generation),
				IPadVariant::Plain { generation } => write!(f, "{}", generation),
				IPadVariant::Pro { size, generation } => write!(f, "Pro {} {}", size, generation),
			}
		}
	}
