use std::hash::Hash;

use crate::prelude::*;

pub use maybe::MaybeScreenSize;

mod maybe {
	use crate::prelude::*;

	/// Wrapper around Option<[ScreenSize]> for easy of [Display] impl.
	/// The display impl *includes a prefix space*.
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct MaybeScreenSize(Option<ScreenSize>);

	impl MaybeScreenSize {
		pub fn as_ref(&self) -> Option<&ScreenSize> {
			self.0.as_ref()
		}

		pub fn get(&self) -> Option<&ScreenSize> {
			self.as_ref()
		}
	}

	impl Display for MaybeScreenSize {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			if let Some(screen_size) = self.0.as_ref() {
				write!(f, " {}", screen_size)
			} else {
				Ok(())
			}
		}
	}

	impl NomFromStr for MaybeScreenSize {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
			map(opt(ScreenSize::nom_from_str), MaybeScreenSize)(input)
		}
	}

	#[cfg(test)]
	mod test {}
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct ScreenSize {
	/// divide by 10 to get number of actual inches
	ten_inches: u16,
	/// whether to show \" or -inch
	/// used to differentiate "11\"" and "11-inch"
	short: bool,
	/// whether to show brackets
	/// used to differentiate "(11-inch)" and "11-inch"
	brackets: bool,
}

impl PartialEq for ScreenSize {
	fn eq(&self, other: &Self) -> bool {
		self.ten_inches == other.ten_inches
	}
}

impl Hash for ScreenSize {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.ten_inches.hash(state);
	}
}

impl ScreenSize {
	/// Long format and brackets
	/// ```rust
	/// use apple_clis::prelude::*;
	///
	/// #[cfg(feature = "unstable-construction")]
	/// {
	/// 	let example = "(11-inch)";
	/// 	let screen_size = ScreenSize::long_brackets(11.0);
	/// 	assert_eq!(format!("{}", screen_size), example);
	/// }
	/// ```
	#[stability::unstable(feature = "construction")]
	pub fn long_brackets(inches: f32) -> Self {
		let inches = inches * 10.0;

		Self {
			ten_inches: inches as u16,
			short: false,
			brackets: true,
		}
	}

	/// Long format and no brackets
	/// ```rust
	/// use apple_clis::prelude::*;
	///
	/// #[cfg(feature = "unstable-construction")]
	/// {
	/// 	let example = "11-inch";
	/// 	let screen_size = ScreenSize::long_bracketless(11.0);
	/// 	assert_eq!(format!("{}", screen_size), example);
	/// }
	/// ```
	#[stability::unstable(feature = "construction")]
	pub fn long_bracketless(inches: f32) -> Self {
		let inches = inches * 10.0;

		Self {
			ten_inches: inches as u16,
			short: false,
			brackets: false,
		}
	}

	/// Short format and brackets
	/// ```rust
	/// use apple_clis::prelude::*;
	///
	/// #[cfg(feature = "unstable-construction")]
	/// {
	/// 	let example = "(13\")";
	/// 	let screen_size = ScreenSize::short_brackets(13.0);
	/// 	assert_eq!(format!("{}", screen_size), example);
	/// }
	/// ```
	#[stability::unstable(feature = "construction")]
	pub fn short_brackets(inches: f32) -> Self {
		let inches = inches * 10.0;

		Self {
			ten_inches: inches as u16,
			short: true,
			brackets: true,
		}
	}

	/// Short format and no brackets
	/// ```rust
	/// use apple_clis::prelude::*;
	///
	/// #[cfg(feature = "unstable-construction")]
	/// {
	/// 	let example = "13\"";
	/// 	let screen_size = ScreenSize::short_bracketless(13.0);
	/// 	assert_eq!(format!("{}", screen_size), example);
	/// }
	/// ```
	#[stability::unstable(feature = "construction")]
	pub fn short_bracketless(inches: f32) -> Self {
		let inches = inches * 10.0;

		Self {
			ten_inches: inches as u16,
			short: true,
			brackets: false,
		}
	}

	pub fn inches(&self) -> f32 {
		self.ten_inches as f32 / 10.0
	}
}

fn brackets_long(input: &str) -> IResult<&str, ScreenSize> {
	delimited(
		ws(tag("(")),
		map(float, ScreenSize::long_brackets),
		ws(tag("-inch)")),
	)(input)
}

fn bracketless_long(input: &str) -> IResult<&str, ScreenSize> {
	terminated(map(float, ScreenSize::long_bracketless), ws(tag("-inch")))(input)
}

fn brackets_short(input: &str) -> IResult<&str, ScreenSize> {
	delimited(
		ws(tag("(")),
		map(float, ScreenSize::short_brackets),
		ws(tag("\")")),
	)(input)
}

fn bracketless_short(input: &str) -> IResult<&str, ScreenSize> {
	terminated(map(float, ScreenSize::short_bracketless), ws(tag("\"")))(input)
}

impl NomFromStr for ScreenSize {
	#[tracing::instrument(level = "trace", skip(input))]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		alt((
			brackets_long,
			brackets_short,
			bracketless_long,
			bracketless_short,
		))(input)
	}
}

impl Display for ScreenSize {
	#[tracing::instrument(level = "trace", skip(self, f))]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match (self.brackets, self.short) {
			(true, false) => write!(f, "({}-inch)", self.inches()),
			(true, true) => write!(f, "({}\")", self.inches()),
			(false, true) => write!(f, "{}\"", self.inches()),
			(false, false) => write!(f, "{}-inch", self.inches()),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::shared::assert_nom_parses;

	use super::*;

	#[test]
	fn screen_size_hardcoded() {
		let examples = ["(5.5-inch)", "(11-inch)", "(6.1\")", "13-inch", "11-inch"];
		assert_nom_parses::<ScreenSize>(examples, |_| true)
	}

	#[test]
	fn screen_size_ordering() {
		let small = ScreenSize::long_brackets(5.0);
		let large = ScreenSize::short_brackets(6.0);
		assert!(large > small);

		let small = ScreenSize::long_brackets(5.0);
		let large = ScreenSize::short_brackets(6.0);
		assert!(large > small);
	}
}
