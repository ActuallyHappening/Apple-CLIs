use anyhow::Context;
use apple_clis::cli::{self, CodeSign, Commands, IosDeploy, Security, Simctl, Spctl, XcRun};
use apple_clis::codesign;
use apple_clis::shared::identifiers::DeviceName;
use apple_clis::shared::ExecInstance;
use apple_clis::xcrun::XcRunInstance;
use apple_clis::{ios_deploy::IosDeployCLIInstance, security, spctl};
use clap::{CommandFactory, Parser};
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
		Commands::GenerateZshCompletions => {
			warn!("This is not tested");
			clap_complete::generate(
				clap_complete::shells::Zsh,
				&mut cli::CliArgs::command(),
				"apple-clis",
				&mut std::io::stdout(),
			);
		}
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
				IosDeploy::Upload { app_path } => {
					let path = match app_path {
						Some(p) => p,
						None => {
							// find directory/file ending in .app
							cli::glob("**/*.app")?
						}
					};
					let devices = ios_deploy_instance.detect_devices()?;
					let device = match devices.first() {
						Some(d) => d,
						None => {
							anyhow::bail!("No devices found to upload to")
						}
					};
					ios_deploy_instance.upload_bundle(device, path)?;
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
				Security::Certs => {
					let teams = security_instance.get_developer_certs()?;
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
							cli::glob("**/*.app")?
						}
					};
					spctl_instance.assess_app(path)?;
				}
			}
		}
		Commands::CodeSign(codesign) => {
			let codesign_instance = codesign::CodesignCLIInstance::try_new_from_which()?;
			match codesign {
				CodeSign::Display { app_path } => {
					let path = match app_path {
						Some(p) => p,
						None => {
							// find directory/file ending in .app
							cli::glob("**/*.app")?
						}
					};
					let output = codesign_instance.display(path)?;
					println!("{}", output);
				}
				CodeSign::Sign { app_path } => {
					let path = match app_path {
						Some(p) => p,
						None => {
							// find directory/file ending in .app
							cli::glob("**/*.app")?
						}
					};
					let security_instance = security::SecurityCLIInstance::try_new_from_which()?;
					let certs = security_instance.get_developer_certs()?;
					let cert = match certs.first() {
						Some(c) => c,
						None => {
							anyhow::bail!("No developer certs found to sign with")
						}
					};
					codesign_instance.sign(cert, path)?;
				}
			}
		}
		Commands::XcRun(xcrun) => {
			let xcrun_instance = XcRunInstance::try_new_from_which()?;
			match xcrun {
				XcRun::Simctl(simctl) => {
					let simctl_instance = xcrun_instance.simctl();
					match simctl {
						Simctl::List => {
							let devices = simctl_instance.list()?;
							let devices = devices.devices().collect::<Vec<_>>();
							println!("{} devices found with `xcrun simctl list`:", devices.len());
							for device in devices {
								println!(
									"Device found: Name = {}, simulator running = {}",
									device.name,
									device.ready()
								);
							}
						}
						Simctl::Boot { ipad, iphone, name } => {
							let device_name: DeviceName = match name {
								Some(n) => n,
								None => {
									let list = simctl_instance.list()?;
									match (ipad, iphone) {
										(true, false) => {
											let latest_ipad = list.ipads().max().context("No simulator iPads found!")?;
											latest_ipad.clone().into()
										}
										(false, true) => {
											let latest_iphone = list.iphones().max().context("No simulator iPhones found!")?;
											latest_iphone.clone().into()
										}
										_ => unreachable!("Clap arguments should prevent this"),
									}
								}
							};
							info!("Booting device: {}", device_name);
							simctl_instance.boot(device_name)?;
						}
					}
				}
			}
		}
	}

	Ok(())
}
