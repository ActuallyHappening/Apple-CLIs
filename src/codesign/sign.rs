use super::CodesignCLIInstance;
use crate::prelude::*;
use crate::security::Certificate;

pub use self::output::*;
mod output {
	use crate::prelude::*;

	#[derive(Debug)]
	pub enum CodeSignSignOutput {
		#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
		UnImplemented(String),
	}

	impl_from_str_nom!(CodeSignSignOutput);

	impl NomFromStr for CodeSignSignOutput {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
			map(rest, |s: &str| CodeSignSignOutput::UnImplemented(s.to_owned()))(input)
		}
	}
}

impl CodesignCLIInstance {
	pub fn sign(&self, cert: &Certificate, path: impl AsRef<Utf8Path>) -> Result<CodeSignSignOutput> {
		let output = self
			.bossy_command()
			.with_arg("-s")
			.with_arg(&cert.common_name)
			.with_arg(path.as_ref())
			.run_and_wait_for_string()?;

		CodeSignSignOutput::from_str(&output)
	}
}
