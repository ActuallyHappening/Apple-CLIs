use apple_clis::{ios_deploy, list_real::list_real};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use tracing::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
	// #[command(flatten)]
	// args: TopLevelCliArgs,
	#[command(subcommand)]
	pub command: Commands,
}

// #[derive(Args, Debug)]
// pub struct TopLevelCliArgs {
//     #[arg(long, env = "CARGO_MANIFEST_DIR")]
//     manifest_path: Utf8PathBuf,
// }

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
	tracing_subscriber::fmt::init();
	let config = CliArgs::parse();

	trace!("Config: {:?}", config);

	match config.command {
		Commands::IosDeployDetect => {
			list_real()?;
		}
		Commands::IosDeploy {
			config,
			bundle_path: bundle_name,
		} => config.execute(&bundle_name)?,
	}

	Ok(())
}
