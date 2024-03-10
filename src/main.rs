use apple_clis::{ios_deploy, list_real::list_real};
use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};
use tracing::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
	#[command(flatten)]
	args: TopLevelCliArgs,

	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Args, Debug)]
pub struct TopLevelCliArgs {
	// #[arg(long, env = "CARGO_MANIFEST_DIR")]
	// manifest_path: Utf8PathBuf,

	#[arg(long)]
	json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	IosDeploy {
		#[command(flatten)]
		config: ios_deploy::IosDeploy,

		bundle_path: Utf8PathBuf,
	},
	IosDeployDetect,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let config = CliArgs::parse();
	if config.human_output() {
		tracing_subscriber::fmt::init();
	}
	
	trace!("Config: {:?}", config);

	match config.command {
		Commands::IosDeployDetect => {
			let devices = list_real()?;
			match config.machine_output() {
				true => {
					serde_json::to_writer(std::io::stdout(), &devices)?;
				}
				false => {
					info!("Note: JSON output can be enabled with the --json flag");
					for device in devices {
						println!("Real device found: {:?}", device);
					}
				}
			}
		}
		Commands::IosDeploy {
			config,
			bundle_path: bundle_name,
		} => config.execute(&bundle_name)?,
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
