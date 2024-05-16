use crate::prelude::*;

pub mod identifiers;
pub mod types;

pub(crate) use traits::*;
pub(crate) use traits::impl_from_str_nom;
pub use traits::PublicCommandOutput;
mod traits;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub(crate) fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
	delimited(multispace0, inner, multispace0)
}

/// Simplify testing
/// ```ignore
/// use apple_clis::prelude::*;
/// 
/// let examples = ["123", "456"]
/// let func = |parsed| parsed.not_unimplemented();
/// assert_nom_parses<YourT>(examples, func);
/// ```
#[cfg(any(test, doctest))]
fn assert_nom_parses<T: NomFromStr + std::fmt::Display + std::fmt::Debug>(
	examples: impl IntoIterator<Item = &'static str>,
	successfully_parsed: impl Fn(&T) -> bool,
) {
	use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

	let fmt_verbose_layer = tracing_subscriber::fmt::layer().pretty();

	tracing_subscriber::registry()
		.with(fmt_verbose_layer)
		.try_init()
		.ok();

	for example in examples.into_iter() {
		let example = example.to_string();
		match T::nom_from_str(&example) {
			Ok((remaining, parsed)) => {
				assert!(
					remaining.is_empty(),
					"Leftover input {:?} while parsing {} into {:?}",
					remaining,
					example,
					parsed
				);
				assert_eq!(
					parsed.to_string(),
					example,
					"Parsing {} into {:?} didn't match Display of {}",
					example,
					parsed,
					example
				);
				assert!(
					successfully_parsed(&parsed),
					"While parsing {}, got unimplemented variant {:?}",
					example,
					parsed
				);
			}
			Err(err) => {
				panic!("Failed to parse {:?}: {}", example, err);
			}
		}
	}
}
