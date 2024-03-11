use apple_clis::ios_deploy::IosDeployInstance;
use camino::Utf8PathBuf;
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

	#[arg(long)]
	json: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
	#[clap(subcommand)]
	IosDeploy(IosDeploy),
}

#[derive(Subcommand, Debug)]
enum IosDeploy {
	/// Spends 5 seconds to detect any already connected devices
	Detect,
}

fn main() -> anyhow::Result<()> {
	let config = CliArgs::parse();
	if config.human_output() {
		tracing_subscriber::fmt::init();
	}

	trace!("Config: {:?}", config);

	match config.command {
		Commands::IosDeploy(ios_deploy) => {
			let ios_deploy_instance = IosDeployInstance::try_new_from_which()?;
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
