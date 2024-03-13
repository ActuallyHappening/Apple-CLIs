use std::num::NonZeroU8;

use nom::{
	character::complete::{digit1, space0},
	combinator::map_res,
	error::ParseError,
	number::complete::float,
	sequence::delimited,
	IResult,
};
use serde::Deserialize;

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);

trait NomFromStr: Sized {
	fn nom_from_str(input: &str) -> IResult<&str, Self>;
}

impl NomFromStr for NonZeroU8 {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		map_res(digit1, |s: &str| s.parse())(input)
	}
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
	delimited(space0, inner, space0)
}

pub mod device_name;
