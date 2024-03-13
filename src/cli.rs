use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use tracing::{info, warn};

use crate::shared::identifiers::DeviceName;

pub mod prelude {
	pub use super::*;
}

#[derive(thiserror::Error, Debug)]
pub enum GlobError {
	#[error("Error converting path to UTF-8: {0}")]
	NonUtf8Paths(#[from] camino::FromPathBufError),

	#[error("Error running glob: {0}")]
	GlobError(#[from] glob::PatternError),

	#[error("No matched files / directories found for pattern: {0}")]
	NoMatchedFiles(String),
}

pub fn glob(pattern: &'static str) -> Result<Utf8PathBuf, GlobError> {
	let matches = glob::glob(pattern)?
		.filter_map(|p| p.ok())
		.filter_map(|p| Utf8PathBuf::try_from(p).ok())
		.collect::<Vec<_>>();

	match matches.first() {
		Some(p) => {
			if matches.len() > 1 {
				warn!(
					"More than one file / directory matched the pattern {:?}, using the first match: {:?}",
					pattern, p
				);
				Ok(p.clone())
			} else {
				info!(
					"Implicitly using the only matched file / directory: {:?}",
					p
				);
				Ok(p.clone())
			}
		}
		None => Err(GlobError::NoMatchedFiles(pattern.into())),
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
	#[arg(long, env = "CARGO_MANIFEST_DIR")]
	manifest_path: Option<Utf8PathBuf>,

	#[arg(long, env = "CARGO")]
	cargo: Option<Utf8PathBuf>,

	#[arg(long)]
	json: bool,
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
	Upload { app_path: Option<Utf8PathBuf> },
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
		app_path: Option<Utf8PathBuf>,
	},
	Sign {
		app_path: Option<Utf8PathBuf>,
	},
}

#[derive(Subcommand, Debug)]
pub enum Spctl {
	AssessApp { app_path: Option<Utf8PathBuf> },
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
		#[arg(long, group = "device_name")]
		ipad: bool,

		#[arg(long, group = "device_name")]
		iphone: bool,

		#[arg(group = "device_name")]
		name: Option<DeviceName>,
	},
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

impl CliArgs {
	pub fn machine_output(&self) -> bool {
		self.args.json
	}

	pub fn human_output(&self) -> bool {
		!self.machine_output()
	}
}
