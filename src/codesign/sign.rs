use super::CodesignCLIInstance;
use crate::prelude::*;
use crate::security::Certificate;

pub use self::output::*;
mod output;

impl CodesignCLIInstance {
	pub fn sign(&self, cert: &Certificate, path: impl AsRef<Utf8Path>) -> Result<SignOutput> {
		SignOutput::from_bossy_result(self
			.bossy_command()
			.with_arg("-s")
			.with_arg(&cert.common_name)
			.with_arg(path.as_ref())
			.run_and_wait_for_output())
	}
}
