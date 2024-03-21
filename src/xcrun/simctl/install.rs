use super::XcRunSimctlInstance;
use crate::prelude::*;

pub use output::*;
mod output;

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
