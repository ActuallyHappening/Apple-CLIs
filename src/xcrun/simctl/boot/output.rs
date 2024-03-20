use crate::prelude::*;

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub enum BootOutput {
	Success,
	AlreadyBooted,

	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	UnImplemented(String),
}

impl BootOutput {
	pub fn success(&self) -> bool {
		matches!(self, BootOutput::Success)
	}
}

fn parse_already_booted(input: &str) -> IResult<&str, BootOutput> {
	let (remaining, _preamble) = ws(tag("An error was encountered processing the command"))(input)?;
	let (remaining, domain) =
		delimited(tag("(domain="), take_till(|c| c == ','), tag(","))(remaining)?;
	let (remaining, error_code) = delimited(ws(tag("code=")), digit1, ws(tag("):")))(remaining)?;
	let (_, msg) =
		all_consuming(ws(tag("Unable to boot device in current state: Booted")))(remaining)?;

	error!(?domain, ?error_code, ?msg, "Parsed xcrun simctl boot error");

	Ok(("", BootOutput::AlreadyBooted))
}

impl NomFromStr for BootOutput {
	fn nom_from_str(input: &str) -> nom::IResult<&str, Self> {
		alt((
			parse_already_booted,
			map(rest, |s: &str| BootOutput::UnImplemented(s.to_owned())),
		))(input)
	}
}

impl_from_str_nom!(BootOutput);

impl BootOutput {
	pub(crate) fn from_output(output: bossy::Result<bossy::Output>) -> Result<Self> {
		match output {
			Ok(_) => Ok(BootOutput::Success),
			Err(err) => match err.output() {
				Some(output) => {
					let stderr = String::from_utf8_lossy(output.stderr());
					BootOutput::from_str(&stderr)
				}
				None => Err(err.into()),
			},
		}
	}
}
