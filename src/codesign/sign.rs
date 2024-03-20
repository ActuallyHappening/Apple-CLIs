use super::CodesignCLIInstance;
use crate::prelude::*;
use crate::security::Certificate;

pub use self::output::*;
mod output {
	use crate::prelude::*;

	#[derive(Debug, Serialize)]
	#[non_exhaustive]
	pub enum SignOutput {
		#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
		UnImplemented(String),
	}

	impl_from_str_nom!(SignOutput);

	impl NomFromStr for SignOutput {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
			map(rest, |s: &str| SignOutput::UnImplemented(s.to_owned()))(input)
		}
	}
}

impl CodesignCLIInstance {
	pub fn sign(&self, cert: &Certificate, path: impl AsRef<Utf8Path>) -> Result<SignOutput> {
		let output = self
			.bossy_command()
			.with_arg("-s")
			.with_arg(&cert.common_name)
			.with_arg(path.as_ref())
			.run_and_wait_for_string()?;

		SignOutput::from_str(&output)
	}
}
