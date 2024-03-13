use std::num::NonZeroU8;

mod prelude {
	pub(super) use super::{ws, NomFromStr};
	#[allow(unused_imports)]
	pub(crate) use nom::{
		branch::alt,
		bytes::complete::{tag, take_till, take_until},
		character::complete::{alpha0, alpha1, digit1, space0, space1},
		combinator::{map, map_res, peek, rest, success, value},
		number::complete::float,
		sequence::tuple,
		sequence::{delimited, preceded, terminated},
		IResult,
	};
	pub(crate) use std::num::NonZeroU8;
}
use nom::error::ParseError;
use prelude::*;

use serde::Deserialize;

pub mod device_name;
pub mod generation;
pub mod screen_size;

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
