use std::fs::File;
use std::io::{BufRead, Write};
use std::ops::Not;

use apple_clis::cli::{self, CodeSign, Commands, Init, IosDeploy, Security, Simctl, Spctl, XcRun};
use apple_clis::codesign;
use apple_clis::open::OpenCLIInstance;
use apple_clis::shared::identifiers::DeviceName;
use apple_clis::xcrun::XcRunInstance;
use apple_clis::{ios_deploy::IosDeployCLIInstance, security, spctl};
use camino::Utf8PathBuf;
use clap::{CommandFactory, Parser};
use color_eyre::eyre::{eyre, Context, ContextCompat};
use serde::Serialize;
use serde_json::json;
use tracing::*;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

fn to_json<T: Serialize>(value: T) -> Result<Option<serde_json::Value>, color_eyre::Report> {
	serde_json::to_value(value)
		.map(Option::Some)
		.map_err(|err| eyre!("Failed to convert value to JSON: {}", err))
}

fn to_raw_json<T: std::fmt::Debug>(
	output: T,
) -> Result<Option<serde_json::Value>, color_eyre::Report> {
	Ok(Some(json!({
		"msg": "This value does not have a good JSON representation yet, so it is not returned in the JSON output. PRs welcome!",
		"raw_output": format!("{:#?}", output),
	})))
}

#[tracing::instrument(level = "trace", skip())]
fn main() {
	let config = cli::CliArgs::parse();

	if config.args.human() {
		// let env_filter = EnvFilter::builder()
		// 	.with_default_directive(LevelFilter::INFO.into())
		// 	.from_env_lossy();
		// tracing_subscriber::fmt().with_env_filter(env_filter).init();

		let fmt_normal_layer = fmt::layer().with_target(false).without_time();
		let fmt_verbose_layer = fmt::layer().pretty();

		let filter_layer = EnvFilter::builder()
			.with_default_directive(LevelFilter::INFO.into())
			.from_env_lossy();

		tracing_subscriber::registry()
			.with(filter_layer)
			.with(config.args.verbose().then_some(fmt_verbose_layer))
			.with(config.args.verbose().not().then_some(fmt_normal_layer))
			.with(tracing_error::ErrorLayer::default())
			.init();

		match color_eyre::install() {
			Ok(_) => {}
			Err(err) => error!(?err, "Could not install `color_eyre` error handler"),
		}
	}

	trace!(config = ?config, "Parsed CLI arguments");

	match run(config.command) {
		Ok(report) => {
			if let Some(report) = report {
				let value = serde_json::ser::to_string_pretty(&report).expect("Couldn't pretty print JSON");
				info!(%value, "Success!");
				if config.args.machine() {
					println!("{}", report);
				}
			}
		}
		Err(e) => {
			let err_msg = format!("{:?}", e);
			error!(error = %err_msg, "Error!");
			std::process::exit(1)
		}
	}
}

#[tracing::instrument(level = "trace", skip(command))]
fn run(command: Commands) -> std::result::Result<Option<serde_json::Value>, color_eyre::Report> {
	match command {
		Commands::Init(Init::NuShell { auto, raw_script }) => match (auto, raw_script) {
			(false, true) => {
				println!("# A script autogenerated by `apple-clis init nushell --raw-script`");
				println!("# To install, check the documentation of `apple-clis init nushell --auto`");
				clap_complete::generate(
					clap_complete_nushell::Nushell,
					&mut cli::CliArgs::command(),
					"apple-clis",
					&mut std::io::stdout(),
				);
				Ok(None)
			}
			(true, false) => {
				// write completions
				{
					// open ~/.apple-clis.nu
					let path = dirs::home_dir()
						.context("No home directory found to install ~/.apple-clis.nu to")?
						.join(".apple-clis.nu");

					let mut completions = Vec::new();
					clap_complete::generate(
						clap_complete_nushell::Nushell,
						&mut cli::CliArgs::command(),
						"apple-clis",
						&mut completions,
					);

					std::fs::write(&path, &completions)
						.context(format!("Failed to write completion script to {:?}", path))?;
					info!(
						"Completion script written to {:?}, now attempting to install it",
						path
					);
				}
				// add source ~/.apple-clis.nu
				{
					let nu_cli_path =
						which::which("nu").context("Couldn't locate `nu` binary on your system")?;
					let config_path = bossy::Command::pure(nu_cli_path)
						.with_args(["-c", "$nu.config-path"])
						.run_and_wait_for_string()
						.context("Running `nu -c '$nu.config-path'` failed")?;
					let config_path = config_path.trim();

					let config_path = Utf8PathBuf::from(config_path);
					let file = File::open(&config_path)
						.context(format!("Cannot open config.nu file at {:?}", &config_path))?;
					let reader = std::io::BufReader::new(file);
					// if there is a line that contains "source ~/.apple-clis.nu" then don't add it
					if reader
						.lines()
						.map_while(Result::ok)
						.any(|line| line.contains("source ~/.apple-clis.nu"))
					{
						info!("~/.apple-clis.nu already sourced in your nu config");
					} else {
						let mut file = std::fs::OpenOptions::new()
							.append(true)
							.open(&config_path)
							.context(format!("Cannot open config.nu file at {:?}", &config_path))?;
						writeln!(file, "source ~/.apple-clis.nu").context(format!(
							"Cannot write to config.nu file at {:?}",
							&config_path
						))?;
						info!("~/.apple-clis.nu added to your nu config");
					}
					Ok(None)
				}
			}
			_ => unreachable!("Clap arguments should prevent this"),
		},
		Commands::IosDeploy(ios_deploy) => {
			let ios_deploy_instance = IosDeployCLIInstance::new()?;
			match ios_deploy {
				IosDeploy::Detect { config } => {
					let devices = ios_deploy_instance.detect_devices(&config)?;
					// println!("{} real devices found with `ios-deploy`:", devices.len());
					// for device in devices {
					// 	println!("Device: {:?}", device);
					// }
					serde_json::to_value(devices)
						.map(Option::Some)
						.map_err(|err| eyre!("Failed to convert devices to JSON: {}", err))
				}
				IosDeploy::Upload {
					app_path,
					auto_detect_config,
				} => {
					let path = app_path.resolve()?;
					let devices = ios_deploy_instance.detect_devices(&auto_detect_config)?;
					let device = match devices.first() {
						Some(d) => d,
						None => Err(eyre!("No devices found to upload to"))?,
					};
					ios_deploy_instance.upload_bundle(device, path)?;
					Ok(None)
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
			let security_instance = security::SecurityCLIInstance::new()?;
			match security {
				Security::Certs => {
					let teams = security_instance.get_developer_certs()?;
					println!("{} development teams found with `security`:", teams.len());
					for team in teams.iter() {
						println!("Team: {:?}", team);
					}
					to_json(teams)
				}
				Security::Pems => {
					let pems = security_instance.get_developer_pems()?;
					println!("{} development pems found with `security`:", pems.len());
					for pem in pems.iter() {
						println!("Pem: {:#?}", pem);
					}
					let debug_str = format!("{:#?}", pems);
					Ok(Some(json!({
							"error": "PEMs don't have a good JSON representation yet, so they are not returned in the JSON output",
							"raw_output": debug_str,
					})))
				}
			}
		}
		Commands::Spctl(spctl) => {
			let spctl_instance = spctl::SpctlCLIInstance::new()?;
			match spctl {
				Spctl::AssessApp { app_path } => {
					let path = app_path.resolve()?;
					let output = spctl_instance.assess_app(path)?;
					to_raw_json(output)
				}
			}
		}
		Commands::CodeSign(codesign) => {
			let codesign_instance = codesign::CodesignCLIInstance::new()?;
			match codesign {
				CodeSign::Display { app_path } => {
					let path = app_path.resolve()?;
					let output = codesign_instance.display(path)?;
					to_json(output)
				}
				CodeSign::Sign { app_path } => {
					let path = app_path.resolve()?;
					let security_instance = security::SecurityCLIInstance::new()?;
					let certs = security_instance.get_developer_certs()?;
					let cert = match certs.first() {
						Some(c) => c,
						None => Err(eyre!("No developer certs found to sign with"))?,
					};
					let output = codesign_instance.sign(cert, path)?;
					to_raw_json(output)
				}
			}
		}
		Commands::XcRun(xcrun) => {
			let xcrun_instance = XcRunInstance::new()?;
			match xcrun {
				XcRun::Simctl(simctl) => {
					let simctl_instance = xcrun_instance.simctl();
					match simctl {
						Simctl::List => {
							let devices = simctl_instance.list()?;
							let devices = devices.devices().collect::<Vec<_>>();
							to_json(devices)
						}
						Simctl::Boot { simulator_id } => {
							let device_name: DeviceName = simulator_id.resolve(&simctl_instance)?;
							info!(simulator_id = %device_name, "Booting device");
							let output = simctl_instance.boot(device_name)?;
							to_json(output)
						}
						Simctl::Install { app_path, booted_simulator } => {
							let path = app_path.resolve()?;
							let booted_simulator = booted_simulator.resolve(&simctl_instance)?;
							simctl_instance.install(path, &booted_simulator)?;
							// simctl_instance.install_booted(path)?;
							Ok(None)
						}
						Simctl::Launch { args } => {
							let config = args.resolve(&simctl_instance)?;
							let output = simctl_instance.launch(&config)?;
							to_raw_json(output)
						}
						Simctl::LaunchBooted { args }  => {
							let config = args.resolve()?;
							let output = simctl_instance.launch_booted(&config)?;
							to_raw_json(output)
						}
					}
				}
			}
		}
		Commands::Open(open) => {
			let open_instance = OpenCLIInstance::new()?;
			let well_known = open.well_known;
			info!(path = ?well_known, "Opening a well known path");
			open_instance.open_well_known(&well_known)?;
			Ok(None)
		}
	}
}
