use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use tracing::{info, warn};

use crate::{open::well_known::WellKnown, shared::identifiers::DeviceName};

use self::{app_path::AppPath, device_name::DeviceSimulator};

pub mod prelude {
	pub use super::*;
}

mod app_path {
	use camino::Utf8PathBuf;
	use color_eyre::eyre::eyre;
	use tracing::{event, info, warn, Level};

	#[derive(clap::Args, Debug)]
	#[group(required = true, multiple = false)]
	pub struct AppPath {
		#[arg(long, long = "path", group = "app_path")]
		app_path: Option<camino::Utf8PathBuf>,

		#[arg(long, group = "app_path")]
		glob: bool,
	}

	impl AppPath {
		pub fn resolve(self) -> Result<Utf8PathBuf, color_eyre::Report> {
			let path = match self.app_path {
				Some(p) => p,
				None => match self.glob {
					false => Err(eyre!(
						"Clap should have enforced that either `app_path` or `glob` was set"
					))?,
					true => {
						let matches = glob::glob("**/*.app")
							.map_err(|err| eyre!("Error running glob: {}", err))?
							.filter_map(|p| p.ok())
							.filter_map(|p| Utf8PathBuf::try_from(p).ok())
							.collect::<Vec<_>>();

						if matches.len() > 1 {
							warn!(
								globbed = ?matches,
								"More than one .app file found, using the first match",
							);
						}

						match matches.first() {
							Some(p) => {
								info!(message = "Using the first matched .app file", "match" = ?p);
								p.clone()
							}
							None => Err(eyre!(
								"No .app files found in the current directory or any subdirectories"
							))?,
						}
					}
				},
			};
			if !path.exists() {
				Err(eyre!("Provided app path does not exist: {:?}", path))?
			}
			Ok(path)
		}
	}
}

mod device_name {
	use color_eyre::eyre::eyre;

	use crate::{prelude::DeviceName, xcrun::simctl::XcRunSimctlInstance};

	#[derive(clap::Args, Debug)]
	#[group(required = true, multiple = false)]
	pub struct DeviceSimulator {
		/// Automatically selects the latest iPad simulator
		#[arg(long, group = "device_name")]
		ipad: bool,

		/// Automatically selects the latest iPhone simulator
		#[arg(long, group = "device_name")]
		iphone: bool,

		/// Provide an exact name, e.g. "iPhone 12 Pro Max"
		#[arg(group = "device_name")]
		name: Option<DeviceName>,
	}

	impl DeviceSimulator {
		pub fn resolve(
			self,
			simctl_instance: &XcRunSimctlInstance,
		) -> Result<DeviceName, color_eyre::Report> {
			match (self.ipad, self.iphone, self.name) {
				(false, false, Some(name)) => Ok(name),
				(true, false, None) => {
					let list = simctl_instance.list()?;
					let latest_ipad = list
						.ipads()
						.max()
						.ok_or_else(|| eyre!("No simulator iPads found!"))?;
					Ok(latest_ipad.clone().into())
				}
				(false, true, None) => {
					let list_output = &simctl_instance.list()?;
					let latest_iphone = list_output
						.iphones()
						.max()
						.ok_or_else(|| eyre!("No simulator iPhones found!"))?;
					Ok(latest_iphone.clone().into())
				}
				_ => Err(eyre!("Clap arguments should prevent this")),
			}
		}
	}
}

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
pub struct TopLevelCliArgs {
	#[arg(long)]
	pub machine: bool,
}

impl CliArgs {
	pub fn machine(&self) -> bool {
		self.args.machine
	}

	pub fn human(&self) -> bool {
		!self.machine()
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

	#[clap(subcommand)]
	Open(Open),
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
	Detect,
	/// Uploads an app to a real device
	Upload {
		#[clap(flatten)]
		app_path: app_path::AppPath,
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
		app_path: AppPath,
	},
	Sign {
		#[clap(flatten)]
		app_path: AppPath,
	},
}

#[derive(Subcommand, Debug)]
pub enum Spctl {
	AssessApp {
		#[clap(flatten)]
		app_path: AppPath,
	},
}

#[derive(Subcommand, Debug)]
pub enum XcRun {
	#[clap(subcommand)]
	Simctl(Simctl),
}

#[derive(Subcommand, Debug)]
pub enum Simctl {
	List,
	#[group(required = true)]
	Boot {
		#[clap(flatten)]
		simulator_id: DeviceSimulator,
	},
	InstallBooted {
		#[clap(flatten)]
		app_path: app_path::AppPath,
	},
}

#[derive(Subcommand, Debug)]
pub enum Open {
	WellKnown {
		#[arg(value_enum)]
		well_known: WellKnown,
	},
}
