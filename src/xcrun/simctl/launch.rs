use crate::prelude::*;

use super::XcRunSimctlInstance;

#[cfg(feature = "cli")]
#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct CliLaunchBootedArgs {
	#[cfg_attr(feature = "cli", clap(long, default_value_t = true))]
	console: bool,

	#[cfg_attr(feature = "cli", clap(flatten))]
	app_path: crate::cli::AppPath,

	#[clap(flatten)]
	simulator_name: crate::cli::DeviceSimulatorBooted,
}

#[derive(Debug)]
pub struct LaunchConfig {
	pub console: bool,
	pub app_path: Utf8PathBuf,
	pub simulator_name: DeviceName,
}

#[cfg(feature = "cli")]
impl CliLaunchBootedArgs {
	pub fn resolve(self, simctl_instance: &XcRunSimctlInstance) -> color_eyre::Result<LaunchConfig> {
		Ok(LaunchConfig {
			console: self.console,
			app_path: self.app_path.resolve()?,
			simulator_name: self.simulator_name.resolve(simctl_instance)?,
		})
	}
}

impl XcRunSimctlInstance<'_> {
	pub fn launch(&self, config: &LaunchConfig) -> error::Result<ExitStatus> {
		let mut cmd = self.bossy_command().with_arg("launch");
		if config.console {
			cmd.add_arg("--console");
		}
		cmd.add_arg(config.simulator_name.to_string());
		cmd.add_arg(config.app_path.as_str());

		Ok(cmd.run_and_wait()?)
	}
}
