use camino::Utf8Path;

use super::CodesignCLIInstance;

#[derive(thiserror::Error, Debug)]
pub enum CodeSignError {
	#[error("Error running `codesign -d`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

impl CodesignCLIInstance {
	pub fn display(&self, path: impl AsRef<Utf8Path>) -> Result<String, CodeSignError> {
		let output = self.bossy_command()
			.with_arg("-d")
			.with_arg(path.as_ref())
			.run_and_wait_for_output()?;
		let stdout = output.stdout_str()?;
		let stderr = output.stderr_str()?;
		Ok(format!("Stdout: {}\nStderr: {}", stdout, stderr))
	}
}