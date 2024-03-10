use std::str::FromStr;

use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use clap::{Args, Parser, Subcommand};

pub mod ios_deploy;
pub mod list_real;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(flatten)]
    args: TopLevelCliArgs,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
pub struct TopLevelCliArgs {
    #[arg(long, env = "CARGO_MANIFEST_DIR")]
    manifest_path: Utf8PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    IosDeploy(ios_deploy::IosDeploy),
		ListReal,
}

#[derive(Debug)]
pub struct RustProjectInfo {
    bundle_identifier: String,
}

impl TopLevelCliArgs {
    pub fn get_cargo_toml(&self) -> anyhow::Result<toml::Value> {
        let cargo_toml_path = self.manifest_path.join("Cargo.toml");
        let cargo_toml = std::fs::read_to_string(&cargo_toml_path)
            .context(format!("Couldn't read Cargo.toml at {}", cargo_toml_path))?;
        toml::from_str(&cargo_toml).context(format!(
            "Cargo.toml is not valid toml at {}",
            cargo_toml_path
        ))
    }

    pub fn get_project_info(&self) -> anyhow::Result<RustProjectInfo> {
        let cargo_toml = self.get_cargo_toml()?;
        let bundle_identifier: String = cargo_toml
            .get("package")
            .context("No package in Cargo.toml")?
            .get("metadata")
            .context("No package.metadata in Cargo.toml")?
            .get("bundle")
            .context("No package.metadata.bundle")?
            .get("package.metadata.bundle.identifier in Cargo.toml")
            .context("package.metadata.bundle.identifier in Cargo.toml is not a string")?
            .to_string();
        Ok(RustProjectInfo { bundle_identifier })
    }
}

#[derive(Debug,)]
pub struct Device {
	id: String,
}

impl FromStr for Device {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Device { id: s.to_string() })
	}
}