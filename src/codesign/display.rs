use super::CodesignCLIInstance;
use crate::prelude::*;

pub use self::output::*;
mod output;

impl CodesignCLIInstance {
	pub fn display(&self, path: impl AsRef<Utf8Path>) -> Result<CodeSignDisplayOutput> {
		let output = self
			.bossy_command()
			.with_arg("-d")
			.with_arg(path.as_ref())
			.run_and_wait_for_output();

		match output {
			Ok(output) => {
				let stdout = String::from_utf8_lossy(output.stdout()).to_string();
				let stderr = String::from_utf8_lossy(output.stderr()).to_string();
				trace!(%stdout, %stderr, "codesign exited successfully");
				Ok(CodeSignDisplayOutput::from_str(&stderr)?)
			}
			Err(err) => {
				match err.output() {
					None => Err(err.into()),
					Some(output) => {
						// handling not signed case
						let stderr = String::from_utf8_lossy(output.stderr());
						CodeSignDisplayOutput::from_str(&stderr)
					}
				}
			}
		}
	}
}
