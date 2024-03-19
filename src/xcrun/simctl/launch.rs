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
	use crate::{prelude::*, shared::resolve_stream};

	#[derive(Debug, Serialize)]
	pub enum LaunchOutput {
		#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/TODO.md"))]
		UnImplemented(String),
	}

	impl_from_str_nom!(LaunchOutput);

	impl NomFromStr for LaunchOutput {
		fn nom_from_str(input: &str) -> IResult<&str, Self> {
			map(rest, |s: &str| LaunchOutput::UnImplemented(s.to_owned()))(input)
		}
	}

	impl TryFrom<bossy::Result<Output>> for LaunchOutput {
		type Error = Error;

		fn try_from(value: bossy::Result<Output>) -> std::result::Result<Self, Self::Error> {
			Self::from_str(&resolve_stream(value)?)
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

		LaunchOutput::try_from(cmd.run_and_wait_for_output())
	}

	pub fn launch_booted(&self, config: &LaunchConfig) -> error::Result<LaunchOutput> {
		let mut cmd = self.bossy_command().with_arg("launch");
		if config.console {
			cmd.add_arg("--console");
		}
		cmd.add_arg("booted");
		cmd.add_arg(config.bundle_id());

		LaunchOutput::try_from(cmd.run_and_wait_for_output())
	}
}
