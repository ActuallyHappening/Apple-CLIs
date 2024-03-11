use apple_clis::cli::{self, Commands, IosDeploy, Security, Spctl};
use apple_clis::{ios_deploy::IosDeployCLIInstance, security, spctl};
use camino::Utf8PathBuf;
use clap::Parser;
use tracing::*;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
	let config = cli::CliArgs::parse();
	if config.human_output() {
		let env_filter = EnvFilter::builder()
			.with_default_directive(LevelFilter::INFO.into())
			.from_env_lossy();
		tracing_subscriber::fmt().with_env_filter(env_filter).init();
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
		Commands::Spctl(spctl) => {
			let spctl_instance = spctl::SpctlCLIInstance::try_new_from_which()?;
			match spctl {
				Spctl::AssessApp { app_path } => {
					let path = match app_path {
						Some(p) => p,
						None => {
							// find directory/file ending in .app
							let paths = glob::glob("**/*.app")?;
							let apps = paths
								.filter_map(|p| p.ok())
								.filter_map(|p| Utf8PathBuf::try_from(p).ok())
								.collect::<Vec<_>>();
							if apps.len() > 1 {
								warn!("More than one .app found in the current directory");
							}
							let app = apps
								.first()
								.ok_or_else(|| anyhow::anyhow!("No .app found in the current directory"))?
								.clone();
							info!(
								"Since no *.app directory / file was passed, implicitly using: {:?}",
								app
							);
							app
						}
					};
					spctl_instance.assess_app(path)?;
				}
			}
		}
	}

	Ok(())
}
