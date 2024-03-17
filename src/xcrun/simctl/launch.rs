use crate::prelude::*;

use super::XcRunSimctlInstance;

#[cfg(feature = "cli")]
#[derive(clap::Args, Debug)]
pub struct CliLaunchArgs {
	#[clap(long, default_value_t = true)]
	console: bool,

	#[clap(flatten)]
	simulator_name: crate::cli::DeviceSimulatorBootedArgs,

	#[clap(flatten)]
	bundle_id: crate::cli::BundleIdentifierArgs,
}

#[cfg(feature = "cli")]
#[derive(clap::Args, Debug)]
pub struct CliLaunchBootedArgs {
	#[clap(long, default_value_t = true)]
	console: bool,

	#[clap(flatten)]
	bundle_id: crate::cli::BundleIdentifierArgs,
}

#[derive(Debug)]
pub struct FullLaunchConfig {
	/// Access using [LaunchConfig]
	booted_config: BootedLaunchConfig,
	pub simulator_name: DeviceName,
}

#[derive(Debug)]
pub struct BootedLaunchConfig {
	pub console: bool,
	pub bundle_id: String,
}

impl FullLaunchConfig {
	pub fn new(console: bool, simulator_name: DeviceName, bundle_id: String) -> Self {
		Self {
			booted_config: BootedLaunchConfig { console, bundle_id },
			simulator_name,
		}
	}

	pub fn simulator_name(&self) -> &DeviceName {
		&self.simulator_name
	}
	pub fn with_simulator_name(mut self, simulator_name: DeviceName) -> Self {
		self.set_simulator_name(simulator_name);
		self
	}
	pub fn set_simulator_name(&mut self, simulator_name: DeviceName) {
		self.simulator_name = simulator_name;
	}
}

impl BootedLaunchConfig {
	pub fn new(console: bool, bundle_id: String) -> Self {
		Self { console, bundle_id }
	}
}

pub trait LaunchConfig: Sized {
	fn console(&self) -> bool;
	fn with_console(mut self, console: bool) -> Self {
		self.set_console(console);
		self
	}
	fn set_console(&mut self, console: bool);

	fn bundle_id(&self) -> &str;
	fn with_bundle_id(mut self, bundle_id: String) -> Self {
		self.set_bundle_id(bundle_id);
		self
	}
	fn set_bundle_id(&mut self, bundle_id: String);
}

impl AsRef<BootedLaunchConfig> for FullLaunchConfig {
	fn as_ref(&self) -> &BootedLaunchConfig {
		&self.booted_config
	}
}

impl AsMut<BootedLaunchConfig> for FullLaunchConfig {
	fn as_mut(&mut self) -> &mut BootedLaunchConfig {
		&mut self.booted_config
	}
}

impl AsRef<BootedLaunchConfig> for BootedLaunchConfig {
	fn as_ref(&self) -> &BootedLaunchConfig {
		self
	}
}

impl AsMut<BootedLaunchConfig> for BootedLaunchConfig {
	fn as_mut(&mut self) -> &mut BootedLaunchConfig {
		self
	}
}

impl<T> LaunchConfig for T
where
	T: AsRef<BootedLaunchConfig> + AsMut<BootedLaunchConfig>,
{
	fn console(&self) -> bool {
		self.as_ref().console
	}
	fn set_console(&mut self, console: bool) {
		self.as_mut().console = console;
	}

	fn bundle_id(&self) -> &str {
		&self.as_ref().bundle_id
	}
	fn set_bundle_id(&mut self, bundle_id: String) {
		self.as_mut().bundle_id = bundle_id;
	}
}

static_assertions::assert_impl_all!(FullLaunchConfig: LaunchConfig);
static_assertions::assert_impl_all!(BootedLaunchConfig: LaunchConfig);

#[cfg(feature = "cli")]
impl CliLaunchArgs {
	pub fn resolve(
		self,
		simctl_instance: &XcRunSimctlInstance,
	) -> color_eyre::Result<FullLaunchConfig> {
		Ok(FullLaunchConfig::new(
			self.console,
			self.simulator_name.resolve(simctl_instance)?,
			self.bundle_id.resolve()?,
		))
	}
}

#[cfg(feature = "cli")]
impl CliLaunchBootedArgs {
	pub fn resolve(self) -> color_eyre::Result<BootedLaunchConfig> {
		Ok(BootedLaunchConfig::new(
			self.console,
			self.bundle_id.resolve()?,
		))
	}
}

impl XcRunSimctlInstance<'_> {
	pub fn launch(&self, config: &FullLaunchConfig) -> error::Result<ExitStatus> {
		let mut cmd = self.bossy_command().with_arg("launch");
		if config.console() {
			cmd.add_arg("--console");
		}
		cmd.add_arg(config.simulator_name().to_string());
		cmd.add_arg(config.bundle_id());

		Ok(cmd.run_and_wait()?)
	}

	pub fn launch_booted(&self, config: &BootedLaunchConfig) -> error::Result<ExitStatus> {
		let mut cmd = self.bossy_command().with_arg("launch");
		if config.console {
			cmd.add_arg("--console");
		}
		cmd.add_arg("booted");
		cmd.add_arg(config.bundle_id());

		Ok(cmd.run_and_wait()?)
	}
}
