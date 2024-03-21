use super::CodesignCLIInstance;
use crate::prelude::*;

pub use self::output::*;
mod output;

impl CodesignCLIInstance {
	#[instrument(skip_all, ret)]
	pub fn display(&self, path: impl AsRef<Utf8Path>) -> Result<DisplayOutput> {
		DisplayOutput::from_bossy_result(
			self
				.bossy_command()
				.with_arg("-d")
				.with_arg(path.as_ref())
				.run_and_wait_for_output(),
		)
	}
}
