use camino::Utf8Path;

use super::SpctlCLIInstance;

#[derive(thiserror::Error, Debug)]
pub enum SpctlAssessError {
	#[error("Error executing `spctl --asses`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

// See https://forums.developer.apple.com/forums/thread/130379
#[derive(Debug)]
enum AssessType {
	App,
	DiskImage,
	InstallerPackage,
}

impl AssessType {
	fn into_type(self) -> &'static str {
		match self {
			AssessType::App => "exec",
			AssessType::DiskImage => "open",
			AssessType::InstallerPackage => "install",
		}
	}
}

impl SpctlCLIInstance {
	fn assess(&self, path: impl AsRef<Utf8Path>, assess_type: AssessType) -> Result<String, SpctlAssessError> {
		Ok(
			self
				.bossy_command()
				.with_arg("--asses")
				.with_args(["--type", assess_type.into_type()])
				.with_arg(path.as_ref())
				.run_and_wait_for_string()?,
		)
	}

	pub fn assess_app(&self, path: impl AsRef<Utf8Path>) -> Result<String, SpctlAssessError> {
		self.assess(path, AssessType::App)
	}
}
