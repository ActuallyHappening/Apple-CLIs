use crate::prelude::*;

use super::XcRunSimctlInstance;

#[cfg(feature = "cli")]
#[derive(Args, Debug)]
pub struct CliLaunchArgs {
	#[clap(long, default_value_t = true)]
	console: bool,

	#[clap(long)]
	bundle_id: String,
}

#[cfg(feature = "cli")]
impl CliLaunchArgs {
	pub fn resolve(self) -> Result<LaunchConfig> {
		Ok(LaunchConfig {
			console: self.console,
			bundle_id: self.bundle_id,
		})
	}
}

#[derive(Debug)]
pub struct LaunchConfig {
	pub console: bool,
	pub bundle_id: String,
}

impl LaunchConfig {
	pub fn console(&self) -> bool {
		self.console
	}

	pub fn bundle_id(&self) -> &str {
		&self.bundle_id
	}
}

pub use output::LaunchOutput;
mod output {
	use crate::prelude::*;

	#[derive(Debug, Serialize)]
	pub enum LaunchOutput {
		#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
		ErrorUnImplemented(String),

		#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
		SuccessUnImplemented(String),
	}

	impl DebugNamed for LaunchOutput {
		fn name() -> &'static str {
			"LaunchOutput"
		}
	}

	impl CommandNomParsable for LaunchOutput {
		fn success_unimplemented(str: String) -> Self {
			Self::SuccessUnImplemented(str)
		}

		fn error_unimplemented(str: String) -> Self {
			Self::ErrorUnImplemented(str)
		}
	}
}

impl XcRunSimctlInstance<'_> {
	pub fn launch(
		&self,
		config: &LaunchConfig,
		simulator_name: DeviceName,
	) -> error::Result<LaunchOutput> {
		let mut cmd = self.bossy_command().with_arg("launch");
		if config.console() {
			cmd.add_arg("--console");
		}
		cmd.add_arg(simulator_name.to_string());
		cmd.add_arg(config.bundle_id());

		LaunchOutput::from_bossy_result(cmd.run_and_wait_for_output())
	}

	pub fn launch_booted(&self, config: &LaunchConfig) -> error::Result<LaunchOutput> {
		let mut cmd = self.bossy_command().with_arg("launch");
		if config.console {
			cmd.add_arg("--console");
		}
		cmd.add_arg("booted");
		cmd.add_arg(config.bundle_id());

		LaunchOutput::from_bossy_result(cmd.run_and_wait_for_output())
	}
}
