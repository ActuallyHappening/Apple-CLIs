use std::ops::Not;

use apple_clis::cli;
use clap::Parser;
use serde_json::json;
use tracing::*;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

#[instrument]
fn main() {
	let config = cli::CliArgs::parse();

	if config.args.human() {
		// let env_filter = EnvFilter::builder()
		// 	.with_default_directive(LevelFilter::INFO.into())
		// 	.from_env_lossy();
		// tracing_subscriber::fmt().with_env_filter(env_filter).init();

		if config.args.verbose() {
			// set env RUST_BACKTRACE=full
			// SAFETY: This is a single-threaded program and does not spawn different threads
			#[allow(unused_unsafe)]
			unsafe {
				std::env::set_var("RUST_BACKTRACE", "full");
				// std::env::set_var("COLORBT_SHOW_HIDDEN", "1");
			};
		}

		let fmt_quiet_layer = fmt::layer().with_target(false).without_time();
		let quiet_filter = EnvFilter::builder()
			.with_default_directive(LevelFilter::WARN.into())
			.from_env_lossy();

		let fmt_normal_layer = fmt::layer().with_target(false).without_time();
		let normal_filter = EnvFilter::builder()
			.with_default_directive(LevelFilter::INFO.into())
			.from_env_lossy();

		let fmt_verbose_layer = fmt::layer().pretty();
		let verbose_filter = EnvFilter::new("info,apple_clis=trace");

		tracing_subscriber::registry()
			.with(if config.args.verbose() {
				verbose_filter
			} else if config.args.quiet() {
				quiet_filter
			} else {
				normal_filter
			})
			.with(config.args.verbose().then_some(fmt_verbose_layer))
			.with(config.args.verbose().not().then_some(fmt_normal_layer))
			.with(config.args.quiet().then_some(fmt_quiet_layer))
			.with(tracing_error::ErrorLayer::default())
			.init();

		match color_eyre::install() {
			Ok(_) => {}
			Err(err) => error!(?err, "Could not install `color_eyre` error handler"),
		}
	}

	trace!(config = ?config, "Parsed CLI arguments");

	match apple_clis::cli::run::run(config.command) {
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
			if config.args.machine() {
				let json = json!({
					"msg": "An error ocurred while running the command",
					"err": err_msg,
				});
				println!(
					"{}",
					serde_json::to_string_pretty(&json).expect("Couldn't pretty print JSON")
				);
			}
			std::process::exit(1)
		}
	}
}
