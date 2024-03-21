use crate::prelude::*;

use crate::{ios_deploy::detect::DetectDevicesConfig, open::well_known::WellKnown, xcrun::simctl};

pub use app_path::AppPathArgs;
mod app_path;
pub use device_name::{DeviceSimulatorBootedArgs, DeviceSimulatorUnBootedArgs};
mod device_name;
pub use bundle_identifier::BundleIdentifierArgs;
mod bundle_identifier;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
	#[command(flatten)]
	pub args: TopLevelCliArgs,

	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
pub struct TopLevelCliArgs {
	/// Outputs data as JSON. Aliased to --json.
	/// Is global, hence can be used with any command/subcommand except help messages
	#[arg(long, global = true, group = "top_level_args", alias = "json")]
	machine: bool,

	/// Overrides the RUST_LOG env variable to use a very verbose log level
	#[arg(long, global = true, group = "top_level_args")]
	verbose: bool,

	/// Only displays warnings and errors
	#[arg(long, short, global = true, group = "top_level_args")]
	quiet: bool,
}

impl TopLevelCliArgs {
	pub fn machine(&self) -> bool {
		self.machine
	}

	pub fn human(&self) -> bool {
		!self.machine()
	}

	pub fn verbose(&self) -> bool {
		self.verbose
	}

	pub fn quiet(&self) -> bool {
		self.quiet
	}
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	/// Used for setting up completions
	#[clap(subcommand)]
	Init(Init),

	#[clap(subcommand)]
	IosDeploy(IosDeploy),

	// #[clap(subcommand)]
	// CargoBundle(CargoBundle),
	#[clap(subcommand)]
	Security(Security),

	#[clap(subcommand)]
	Spctl(Spctl),

	#[clap(subcommand, name = "codesign")]
	CodeSign(CodeSign),

	#[clap(subcommand, name = "xcrun")]
	XcRun(XcRun),

	Open(#[clap(flatten)] Open),
}

#[derive(Subcommand, Debug)]
pub enum Init {
	#[clap(name = "nushell")]
	#[group(required = true)]
	NuShell {
		/// Automatically adds shell completions to `~/.apple-clis.nu`
		/// and configures your config.nu file to source it.
		/// This is the recommended approach.
		#[arg(long, group = "init")]
		auto: bool,

		/// Run `apple-clis init nushell --raw-script | save -f ~/.apple-clis.nu`
		/// Then add `source ~/.apple-clis.nu` to your config.nu file,
		/// E.g. `"source ~/.apple-clis.nu" | save --append $nu.config-path`
		#[arg(long, group = "init")]
		raw_script: bool,
	},
}

#[derive(Subcommand, Debug)]
pub enum IosDeploy {
	/// Spends a second to detect any already connected devices
	Detect {
		#[clap(flatten)]
		config: DetectDevicesConfig,
	},
	/// Uploads an app to a real device
	Upload {
		#[clap(flatten)]
		app_path: app_path::AppPathArgs,

		#[clap(flatten)]
		auto_detect_config: DetectDevicesConfig,
	},
}

#[derive(Subcommand, Debug)]
pub enum CargoBundle {
	/// Bundles the iOS app
	Ios,
}

#[derive(Subcommand, Debug)]
pub enum Security {
	Certs,
	Pems,
}

#[derive(Subcommand, Debug)]
pub enum CodeSign {
	/// Displays the code signature of the given file
	Display {
		#[clap(flatten)]
		app_path: AppPathArgs,
	},
	Sign {
		#[clap(flatten)]
		app_path: AppPathArgs,
	},
}

#[derive(Subcommand, Debug)]
pub enum Spctl {
	AssessApp {
		#[clap(flatten)]
		app_path: AppPathArgs,
	},
}

#[derive(Subcommand, Debug)]
pub enum XcRun {
	#[clap(subcommand)]
	Simctl(Simctl),
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Booted {
	Booted,
}

#[derive(Subcommand, Debug)]
pub enum Simctl {
	List,
	Boot {
		#[clap(flatten)]
		simulator_id: DeviceSimulatorUnBootedArgs,
	},
	Install {
		#[clap(flatten)]
		booted_simulator: DeviceSimulatorBootedArgs,

		#[clap(flatten)]
		app_path: app_path::AppPathArgs,
	},
	Launch {
		#[clap(flatten)]
		booted_simulator: DeviceSimulatorBootedArgs,

		#[clap(flatten)]
		args: simctl::launch::CliLaunchArgs,
	},
}

#[derive(Args, Debug)]
pub struct Open {
	#[arg(long, value_enum)]
	pub well_known: WellKnown,
}
