use crate::prelude::*;

use super::XcRunSimctlInstance;

#[cfg(feature = "cli")]
#[derive(Args, Debug)]
pub struct CliLaunchArgs {
	#[clap(long, default_value_t = true)]
	console: bool,

	#[clap(long)]
	bundle_id: String,

	/// If true, pipes the spawned process's console to the current process's console.
	/// This is the default behavior.
	#[clap(long, default_value_t = true)]
	piped: bool,
}

#[cfg(feature = "cli")]
impl CliLaunchArgs {
	pub fn resolve(self) -> Result<(bool, LaunchConfig)> {
		let config = LaunchConfig {
			console: self.console,
			bundle_id: self.bundle_id,
		};
		Ok((self.piped, config))
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
mod output;

impl XcRunSimctlInstance<'_> {
	#[instrument(ret, skip(self,))]
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

	/// Like [Self::launch], but pipes the console of the launched
	/// process to the current process's console.
	///
	/// The upshot of this is you can see your programs logs in the console!
	///
	/// Not setting [LaunchConfig::console] to true will result in a warning,
	/// since presumably you are calling this function over [Self::launch] to
	/// see logs in the console.
	#[instrument(skip_all, ret, fields(?config, %simulator_name))]
	pub fn launch_piped(
		&self,
		config: &LaunchConfig,
		simulator_name: DeviceName,
	) -> Result<ExitStatus> {
		let mut cmd = self.bossy_command().with_arg("launch");
		if config.console() {
			cmd.add_arg("--console");
		} else {
			warn!(
				?config,
				?simulator_name,
				"Why are you calling launch_piped without console set to true?"
			);
		}
		cmd.add_arg(simulator_name.to_string());
		cmd.add_arg(config.bundle_id());

		Ok(cmd.run_and_wait()?)
	}
}
