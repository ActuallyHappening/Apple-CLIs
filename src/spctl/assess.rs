use crate::prelude::*;

use camino::Utf8Path;

use super::SpctlCLIInstance;

// See https://forums.developer.apple.com/forums/thread/130379
#[derive(Debug)]
pub enum AssessType {
	App,
	DiskImage,
	InstallerPackage,
}

impl AssessType {
	#[tracing::instrument(level = "trace", skip(self))]
	fn into_type(self) -> &'static str {
		match self {
			AssessType::App => "exec",
			AssessType::DiskImage => "open",
			AssessType::InstallerPackage => "install",
		}
	}
}

impl SpctlCLIInstance {
	#[tracing::instrument(level = "trace", skip(self, path, assess_type))]
	#[doc = include_str!("../../docs/inline/TODO.md")]
	pub fn assess(&self, path: impl AsRef<Utf8Path>, assess_type: AssessType) -> Result<String> {
		Ok(
			self
				.bossy_command()
				.with_arg("--asses")
				.with_args(["--type", assess_type.into_type()])
				.with_arg(path.as_ref())
				.run_and_wait_for_string()?,
		)
	}

	#[tracing::instrument(level = "trace", skip(self, path))]
	pub fn assess_app(&self, path: impl AsRef<Utf8Path>) -> Result<String> {
		self.assess(path, AssessType::App)
	}
}
