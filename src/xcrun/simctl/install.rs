use super::XcRunSimctlInstance;
use crate::prelude::*;

pub use output::*;
mod output {
	use crate::prelude::*;

	#[derive(Debug, Serialize)]
	pub enum InstallOutput {
		#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
		SuccessUnImplemented(String),

		#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
		ErrorUnImplemented(String),
	}
	impl DebugNamed for InstallOutput {
		fn name() -> &'static str {
			"InstallOutput"
		}
	}

	impl CommandNomParsable for InstallOutput {
		fn success_unimplemented(str: String) -> Self {
			Self::SuccessUnImplemented(str)
		}

		fn error_unimplemented(str: String) -> Self {
			Self::ErrorUnImplemented(str)
		}
	}
}

impl XcRunSimctlInstance<'_> {
	#[instrument(skip_all, ret)]
	pub fn install_booted(&self, app_path: impl AsRef<Utf8Path>) -> Result<InstallOutput> {
		let app_path = app_path.as_ref();
		InstallOutput::from_bossy_result(
			self
				.bossy_command()
				.with_arg("install")
				.with_arg("booted")
				.with_arg(app_path)
				.run_and_wait_for_output(),
		)
	}

	#[instrument(skip_all, ret)]
	pub fn install(
		&self,
		app_path: impl AsRef<Utf8Path>,
		booted_simulator: &DeviceName,
	) -> Result<InstallOutput> {
		let app_path = app_path.as_ref();
		InstallOutput::from_bossy_result(
			self
				.bossy_command()
				.with_arg("install")
				.with_arg(booted_simulator.to_string())
				.with_arg(app_path)
				.run_and_wait_for_output(),
		)
	}
}
