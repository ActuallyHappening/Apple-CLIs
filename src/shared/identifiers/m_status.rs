use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MStatus(NonZeroU8);

/// Wrapper around `Option<MStatus>` to allow for convenient [Display] impl.
/// *The [Display] impl has a prefix space!*
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MaybeMStatus(Option<MStatus>);

impl MStatus {
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
	fn default_testing() -> Self {
		Self(NonZeroU8::new(1).unwrap())
	}
}

impl Display for MaybeMStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.0 {
			Some(m) => write!(f, " {}", m),
			None => Ok(()),
		}
	}
}

impl MaybeMStatus {
	#[allow(dead_code)]
	pub fn get(&self) -> Option<&MStatus> {
		self.0.as_ref()
	}

	pub(crate) fn new(m: Option<MStatus>) -> Self {
		Self(m)
	}

	#[cfg(test)]
	pub(crate) fn default_testing() -> Self {
		Self::new(Some(MStatus::default_testing()))
	}
}

/// parses (M1) -> MStatus(NonZeroU8(1))
fn parse_m_status(input: &str) -> IResult<&str, MStatus> {
	delimited(
		tag("(M"),
		map(NonZeroU8::nom_from_str, MStatus::new),
		tag(")"),
	)(input)
}

impl Display for MStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "(M{})", self.get())
	}
}

impl NomFromStr for MaybeMStatus {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		map(alt((
			map(parse_m_status, Some),
			value(None, tag("")),
		)), MaybeMStatus::new)(input)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_hard_coded() {
		let examples = [
			"(M1)",
			"(M2)",
		];
		for example in examples {
			let (remaining, parsed) = parse_m_status(example).unwrap();
			assert!(remaining.is_empty());
			assert_eq!(parsed.to_string(), example);
		}
	}
}