use apple_clis::{cargo_bundle, ios_deploy::IosDeployCLIInstance, security};
use camino::{Utf8Path, Utf8PathBuf};
use clap::{Args, Parser, Subcommand};
use tracing::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct CliArgs {
	#[command(flatten)]
	args: TopLevelCliArgs,

	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Args, Debug)]
struct TopLevelCliArgs {
	#[arg(long, env = "CARGO_MANIFEST_DIR")]
	manifest_path: Option<Utf8PathBuf>,

	#[arg(long, env = "CARGO")]
	cargo: Option<Utf8PathBuf>,

	#[arg(long)]
	json: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
	#[clap(subcommand)]
	IosDeploy(IosDeploy),

	// #[clap(subcommand)]
	// CargoBundle(CargoBundle),

	#[clap(subcommand)]
	Security(Security)
}

#[derive(Subcommand, Debug)]
enum IosDeploy {
	/// Spends 5 seconds to detect any already connected devices
	Detect,
}

#[derive(Subcommand, Debug)]
enum CargoBundle {
	/// Bundles the iOS app
	Ios,
}

#[derive(Subcommand, Debug)]
enum Security {
	Teams,
	Pems,
}

#[derive(thiserror::Error, Debug)]
pub enum TopLevelArgsError {
	#[error("Error getting current working directory")]
	CwdDoesNotExist(std::io::Error),

	#[error("Error converting CWD path to UTF-8: {0}")]
	PathNotUtf8(#[from] camino::FromPathBufError),

	#[error("The CWD does not contain a `Cargo.toml` file")]
	CargoTomlDoesNotExist,

	#[error("Error running `which cargo`: {0}")]
	CannotWhichCargo(#[from] which::Error),
}

impl TopLevelCliArgs {
	fn try_get_manifest_path(&self) -> Result<Utf8PathBuf, TopLevelArgsError> {
		match &self.manifest_path {
			Some(p) => Ok(p.clone()),
			None => match std::env::current_dir() {
				Ok(p) => {
					if p.join("Cargo.toml").exists() {
						Ok(Utf8PathBuf::try_from(p)?)
					} else {
						Err(TopLevelArgsError::CargoTomlDoesNotExist)
					}
				}
				Err(err) => Err(TopLevelArgsError::CwdDoesNotExist(err)),
			},
		}
	}

	fn try_get_cargo_path(&self) -> Result<Utf8PathBuf, TopLevelArgsError> {
		match &self.cargo {
			Some(p) => Ok(p.clone()),
			None => Ok(Utf8PathBuf::try_from(which::which("cargo")?)?),
		}
	}
}

fn main() -> anyhow::Result<()> {
	let config = CliArgs::parse();
	if config.human_output() {
		tracing_subscriber::fmt::init();
	}

	trace!("Config: {:?}", config);

	match config.command {
		Commands::IosDeploy(ios_deploy) => {
			let ios_deploy_instance = IosDeployCLIInstance::try_new_from_which()?;
			match ios_deploy {
				IosDeploy::Detect => {
					let devices = ios_deploy_instance.detect_devices()?;
					println!("{} real devices found with `ios-deploy`:", devices.len());
					for device in devices {
						println!("Device: {:?}", device);
					}
				}
			}
		}
		// Commands::CargoBundle(cargo_bundle) => {
		// 	let cargo_bundle_instance = cargo_bundle::CargoBundleInstance::new(
		// 		config.args.try_get_cargo_path()?,
		// 		// config.args.try_get_manifest_path()?,
		// 	);
		// 	// set cwd to manifest dir
		// 	std::env::set_current_dir(config.args.try_get_manifest_path()?)?;
		// 	match cargo_bundle {
		// 		CargoBundle::Ios => {
		// 			cargo_bundle_instance.bundle_ios()?;
		// 		}
		// 	}
		// }
		Commands::Security(security) => {
			let security_instance = security::SecurityCLIInstance::try_new_from_which()?;
			match security {
				Security::Teams => {
					let teams = security_instance.get_developer_teams()?;
					println!("{} development teams found with `security`:", teams.len());
					for team in teams {
						println!("Team: {:?}", team);
					}
				}
				Security::Pems => {
					let pems = security_instance.get_developer_pems()?;
					println!("{} development pems found with `security`:", pems.len());
					for pem in pems {
						println!("Pem: {:#?}", pem);
					}
				}
			}
		}
	}

	Ok(())
}

impl CliArgs {
	pub fn machine_output(&self) -> bool {
		self.args.json
	}

	pub fn human_output(&self) -> bool {
		!self.machine_output()
	}
}
