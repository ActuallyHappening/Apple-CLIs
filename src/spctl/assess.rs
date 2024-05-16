use crate::prelude::*;

use super::SpctlCLIInstance;

pub use output::*;
mod output;

impl SpctlCLIInstance {
	#[instrument(skip_all, ret)]
	#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
	pub fn assess(
		&self,
		path: impl AsRef<Utf8Path>,
		assess_type: AssessType,
	) -> Result<AssessOutput> {
		AssessOutput::from_bossy_result(
			self
				.bossy_command()
				.with_arg("--asses")
				.with_args(["--type", assess_type.into_type()])
				.with_arg(path.as_ref())
				.run_and_wait_for_output(),
		)
	}

	pub fn assess_app(&self, path: impl AsRef<Utf8Path>) -> Result<AssessOutput> {
		self.assess(path, AssessType::App)
	}
}

/// See <https://forums.developer.apple.com/forums/thread/130379>
#[derive(Debug)]
pub enum AssessType {
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
