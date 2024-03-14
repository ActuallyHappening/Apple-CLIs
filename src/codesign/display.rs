use crate::prelude::*;

use camino::Utf8Path;

use super::CodesignCLIInstance;

impl CodesignCLIInstance {
	#[doc = include_str!("../../docs/inline/TODO.md")]
	pub fn display(&self, path: impl AsRef<Utf8Path>) -> Result<String> {
		let output = self
			.bossy_command()
			.with_arg("-d")
			.with_arg(path.as_ref())
			.run_and_wait_for_output()?;
		let stdout = output.stdout_str()?;
		let stderr = output.stderr_str()?;
		Ok(format!("Stdout: {}\nStderr: {}", stdout, stderr))
	}
}
